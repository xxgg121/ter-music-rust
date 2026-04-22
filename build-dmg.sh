#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: ./build-dmg.sh [options]

Build macOS app bundle for ter-music-rust.

Options:
  --target <triple>      Rust target triple (x86_64-apple-darwin | aarch64-apple-darwin)
  --universal            Build universal app (merge x86_64 + aarch64 with lipo)
  --skip-build           Skip cargo build and package existing binaries
  --no-zip               Do not generate zip archive
  --with-dmg             Generate dmg image (requires hdiutil)
  --output-dir <dir>     Output directory (default: ./dist/macos)
  -h, --help             Show this help

Examples:
  ./build-dmg.sh
  ./build-dmg.sh --target aarch64-apple-darwin
  ./build-dmg.sh --universal --with-dmg
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"
OUTPUT_DIR="$PROJECT_DIR/dist/macos"
WORK_DIR="$PROJECT_DIR/target/macos-package"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Error: build-dmg.sh must run on macOS (Darwin)." >&2
  echo "Hint: On Linux, use ./build-deb.sh or ./build-rpm.sh." >&2
  exit 1
fi

UNIVERSAL=0
SKIP_BUILD=0
WITH_ZIP=1
WITH_DMG=0

HOST_ARCH="$(uname -m)"
if [[ "$HOST_ARCH" == "arm64" || "$HOST_ARCH" == "aarch64" ]]; then
  TARGET="aarch64-apple-darwin"
else
  TARGET="x86_64-apple-darwin"
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET="${2:-}"
      [[ -n "$TARGET" ]] || { echo "Error: --target requires a value" >&2; exit 1; }
      shift 2
      ;;
    --universal)
      UNIVERSAL=1
      shift
      ;;
    --skip-build)
      SKIP_BUILD=1
      shift
      ;;
    --no-zip)
      WITH_ZIP=0
      shift
      ;;
    --with-dmg)
      WITH_DMG=1
      shift
      ;;
    --output-dir)
      OUTPUT_DIR="${2:-}"
      [[ -n "$OUTPUT_DIR" ]] || { echo "Error: --output-dir requires a value" >&2; exit 1; }
      shift 2
      ;;
    -h|--help)
      show_help
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      show_help >&2
      exit 1
      ;;
  esac
done

command -v cargo >/dev/null 2>&1 || { echo "Error: cargo not found" >&2; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "Error: python3 not found" >&2; exit 1; }
if [[ "$WITH_ZIP" -eq 1 ]]; then
  command -v ditto >/dev/null 2>&1 || { echo "Error: ditto not found" >&2; exit 1; }
fi

if [[ "$UNIVERSAL" -eq 1 ]] && ! command -v lipo >/dev/null 2>&1; then
  echo "Error: lipo not found (Xcode Command Line Tools required)" >&2
  exit 1
fi

PKG_NAME="$(python3 - <<'PY' "$CARGO_TOML"
import pathlib,re,sys
text=pathlib.Path(sys.argv[1]).read_text(encoding='utf-8')
print(re.search(r'^name\s*=\s*"([^"]+)"', text, re.M).group(1))
PY
)"
PKG_VERSION="$(python3 - <<'PY' "$CARGO_TOML"
import pathlib,re,sys
text=pathlib.Path(sys.argv[1]).read_text(encoding='utf-8')
print(re.search(r'^version\s*=\s*"([^"]+)"', text, re.M).group(1))
PY
)"

APP_DISPLAY_NAME="Ter Music Rust"
APP_BUNDLE_NAME="${APP_DISPLAY_NAME}.app"
APP_BUNDLE_ID="com.xxgg121.ter-music-rust"
ICON_SOURCE="$PROJECT_DIR/assets/icons/ter-music-rust-512.png"

rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR" "$OUTPUT_DIR"

build_target() {
  local t="$1"
  rustup target add "$t" >/dev/null 2>&1 || true
  cargo build --release --target "$t"
}

if [[ "$SKIP_BUILD" -eq 0 ]]; then
  if [[ "$UNIVERSAL" -eq 1 ]]; then
    build_target "x86_64-apple-darwin"
    build_target "aarch64-apple-darwin"
  else
    build_target "$TARGET"
  fi
fi

APP_DIR="$WORK_DIR/$APP_BUNDLE_NAME"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
ICONSET_DIR="$WORK_DIR/icon.iconset"

mkdir -p "$MACOS_DIR" "$RESOURCES_DIR"

if [[ "$UNIVERSAL" -eq 1 ]]; then
  BIN_X64="$PROJECT_DIR/target/x86_64-apple-darwin/release/$PKG_NAME"
  BIN_ARM64="$PROJECT_DIR/target/aarch64-apple-darwin/release/$PKG_NAME"
  [[ -f "$BIN_X64" ]] || { echo "Error: binary not found: $BIN_X64" >&2; exit 1; }
  [[ -f "$BIN_ARM64" ]] || { echo "Error: binary not found: $BIN_ARM64" >&2; exit 1; }
  lipo -create -output "$RESOURCES_DIR/$PKG_NAME" "$BIN_X64" "$BIN_ARM64"
else
  BIN_PATH="$PROJECT_DIR/target/$TARGET/release/$PKG_NAME"
  [[ -f "$BIN_PATH" ]] || { echo "Error: binary not found: $BIN_PATH" >&2; exit 1; }
  cp "$BIN_PATH" "$RESOURCES_DIR/$PKG_NAME"
fi
chmod 0755 "$RESOURCES_DIR/$PKG_NAME"

# App launcher: open Terminal and run the real TUI binary
cat > "$MACOS_DIR/$PKG_NAME" <<EOF
#!/usr/bin/env bash
set -e
APP_ROOT="\$(cd "\$(dirname "\$0")/.." && pwd)"
APP_BIN="\$APP_ROOT/Resources/$PKG_NAME"

if command -v osascript >/dev/null 2>&1; then
  exec osascript - "\$APP_BIN" <<'APPLESCRIPT'
on run argv
  set binPath to item 1 of argv
  tell application "Terminal"
    activate
    do script quoted form of binPath
  end tell
end run
APPLESCRIPT
fi

exec "\$APP_BIN"
EOF
chmod 0755 "$MACOS_DIR/$PKG_NAME"

# Generate .icns if possible
if [[ -f "$ICON_SOURCE" ]] && command -v iconutil >/dev/null 2>&1 && command -v sips >/dev/null 2>&1; then
  rm -rf "$ICONSET_DIR"
  mkdir -p "$ICONSET_DIR"
  sips -z 16 16     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_16x16.png" >/dev/null
  sips -z 32 32     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_16x16@2x.png" >/dev/null
  sips -z 32 32     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_32x32.png" >/dev/null
  sips -z 64 64     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_32x32@2x.png" >/dev/null
  sips -z 128 128   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_128x128.png" >/dev/null
  sips -z 256 256   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_128x128@2x.png" >/dev/null
  sips -z 256 256   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_256x256.png" >/dev/null
  sips -z 512 512   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_256x256@2x.png" >/dev/null
  sips -z 512 512   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_512x512.png" >/dev/null
  cp "$ICON_SOURCE" "$ICONSET_DIR/icon_512x512@2x.png"
  iconutil -c icns "$ICONSET_DIR" -o "$RESOURCES_DIR/ter-music-rust.icns"
fi

cat > "$CONTENTS_DIR/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>${APP_DISPLAY_NAME}</string>
  <key>CFBundleExecutable</key>
  <string>${PKG_NAME}</string>
  <key>CFBundleIconFile</key>
  <string>ter-music-rust</string>
  <key>CFBundleIdentifier</key>
  <string>${APP_BUNDLE_ID}</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>${APP_DISPLAY_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${PKG_VERSION}</string>
  <key>CFBundleVersion</key>
  <string>${PKG_VERSION}</string>
  <key>LSMinimumSystemVersion</key>
  <string>10.13</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

APP_OUT="$OUTPUT_DIR/${APP_BUNDLE_NAME}"
rm -rf "$APP_OUT"
cp -R "$APP_DIR" "$APP_OUT"

ZIP_OUT="$OUTPUT_DIR/${PKG_NAME}-${PKG_VERSION}-macos.zip"
if [[ "$WITH_ZIP" -eq 1 ]]; then
  rm -f "$ZIP_OUT"
  ditto -c -k --sequesterRsrc --keepParent "$APP_OUT" "$ZIP_OUT"
fi

DMG_OUT="$OUTPUT_DIR/${PKG_NAME}-${PKG_VERSION}-macos.dmg"
if [[ "$WITH_DMG" -eq 1 ]]; then
  command -v hdiutil >/dev/null 2>&1 || { echo "Error: hdiutil not found" >&2; exit 1; }
  DMG_STAGE="$WORK_DIR/dmg-stage"
  rm -rf "$DMG_STAGE"
  mkdir -p "$DMG_STAGE"
  cp -R "$APP_OUT" "$DMG_STAGE/"
  rm -f "$DMG_OUT"
  hdiutil create -volname "$APP_DISPLAY_NAME" -srcfolder "$DMG_STAGE" -ov -format UDZO "$DMG_OUT" >/dev/null
fi

echo "macOS package build completed: $OUTPUT_DIR"
echo "App bundle: $APP_OUT"
if [[ "$WITH_ZIP" -eq 1 ]]; then
  echo "ZIP: $ZIP_OUT"
fi
if [[ "$WITH_DMG" -eq 1 ]]; then
  echo "DMG: $DMG_OUT"
fi
