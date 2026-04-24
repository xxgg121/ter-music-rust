#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: ./build-deb.sh [options]

Build Debian packages for ter-music-rust.

Options:
  --with-debuginfo      Build and generate additional -dbg package
  --with-source         Generate source package using dpkg-source
  --target <triple>     Rust target triple (default: x86_64-unknown-linux-gnu)
  --arch <arch>         Debian architecture (default: amd64)
  --output-dir <dir>    Output directory for generated packages (default: ./dist/deb)
  -h, --help            Show this help message

Examples:
  ./build-deb.sh
  ./build-deb.sh --with-debuginfo
  ./build-deb.sh --with-source --with-debuginfo
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"

WITH_DEBUGINFO=0
WITH_SOURCE=0
TARGET="x86_64-unknown-linux-gnu"
DEB_ARCH="amd64"
OUTPUT_DIR="$PROJECT_DIR/dist/deb"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-debuginfo)
      WITH_DEBUGINFO=1
      shift
      ;;
    --with-source)
      WITH_SOURCE=1
      shift
      ;;
    --target)
      TARGET="${2:-}"
      [[ -n "$TARGET" ]] || { echo "Error: --target requires a value" >&2; exit 1; }
      shift 2
      ;;
    --arch)
      DEB_ARCH="${2:-}"
      [[ -n "$DEB_ARCH" ]] || { echo "Error: --arch requires a value" >&2; exit 1; }
      shift 2
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
command -v dpkg-deb >/dev/null 2>&1 || { echo "Error: dpkg-deb not found" >&2; exit 1; }
command -v dpkg-source >/dev/null 2>&1 || { echo "Error: dpkg-source not found" >&2; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "Error: python3 not found" >&2; exit 1; }
command -v pkg-config >/dev/null 2>&1 || { echo "Error: pkg-config not found (install pkg-config)" >&2; exit 1; }

if ! pkg-config --exists alsa; then
  cat >&2 <<'EOF'
Error: ALSA development files not found (alsa.pc).
Install the package providing alsa.pc, then re-run:
  Debian/Ubuntu: sudo apt update && sudo apt install -y libasound2-dev pkg-config
  Fedora/RHEL:   sudo dnf install -y alsa-lib-devel pkgconf-pkg-config
  Arch:          sudo pacman -S --needed alsa-lib pkgconf
EOF
  exit 1
fi

if [[ "$WITH_DEBUGINFO" -eq 1 ]]; then
  command -v objcopy >/dev/null 2>&1 || { echo "Error: objcopy not found (install binutils)" >&2; exit 1; }
  command -v strip >/dev/null 2>&1 || { echo "Error: strip not found (install binutils)" >&2; exit 1; }
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
PKG_MAINTAINER="Ter Music Maintainers <noreply@example.invalid>"
PKG_SECTION="sound"
PKG_PRIORITY="optional"
PKG_HOMEPAGE="https://github.com/xxgg121/ter-music-rust"
ICON_DIR="$PROJECT_DIR/assets/icons"
ICON_FALLBACK="$ICON_DIR/ter-music-rust.png"

WORK_DIR="$PROJECT_DIR/target/deb-build"
BIN_PKG_DIR="$WORK_DIR/${PKG_NAME}_${PKG_VERSION}_${DEB_ARCH}"
DBG_PKG_NAME="${PKG_NAME}-dbg"
DBG_PKG_DIR="$WORK_DIR/${DBG_PKG_NAME}_${PKG_VERSION}_${DEB_ARCH}"
BINARY_PATH="$PROJECT_DIR/target/${TARGET}/release/${PKG_NAME}"

rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR" "$OUTPUT_DIR"

if [[ "$WITH_DEBUGINFO" -eq 1 ]]; then
  CARGO_PROFILE_RELEASE_STRIP=none RUSTFLAGS="-C debuginfo=2" cargo build --release --target "$TARGET"
else
  cargo build --release --target "$TARGET"
fi

[[ -f "$BINARY_PATH" ]] || { echo "Error: binary not found at $BINARY_PATH" >&2; exit 1; }

mkdir -p "$BIN_PKG_DIR/DEBIAN" \
         "$BIN_PKG_DIR/usr/bin" \
         "$BIN_PKG_DIR/usr/share/doc/$PKG_NAME" \
         "$BIN_PKG_DIR/usr/share/applications" \
         "$BIN_PKG_DIR/usr/share/pixmaps"

for sz in 96 128 256 512; do
  mkdir -p "$BIN_PKG_DIR/usr/share/icons/hicolor/${sz}x${sz}/apps"
done

install -m 0755 "$BINARY_PATH" "$BIN_PKG_DIR/usr/bin/$PKG_NAME"

for sz in 96 128 256 512; do
  icon_file="$ICON_DIR/ter-music-rust-${sz}.png"
  if [[ -f "$icon_file" ]]; then
    install -m 0644 "$icon_file" "$BIN_PKG_DIR/usr/share/icons/hicolor/${sz}x${sz}/apps/${PKG_NAME}.png"
  fi
done

if [[ -f "$ICON_FALLBACK" ]]; then
  install -m 0644 "$ICON_FALLBACK" "$BIN_PKG_DIR/usr/share/pixmaps/${PKG_NAME}.png"
elif [[ -f "$ICON_DIR/ter-music-rust-512.png" ]]; then
  install -m 0644 "$ICON_DIR/ter-music-rust-512.png" "$BIN_PKG_DIR/usr/share/pixmaps/${PKG_NAME}.png"
fi

cat > "$BIN_PKG_DIR/usr/bin/${PKG_NAME}-launcher" <<EOF
#!/usr/bin/env bash
set -e
APP_BIN="/usr/bin/${PKG_NAME}"

if command -v x-terminal-emulator >/dev/null 2>&1; then
  exec x-terminal-emulator -e "\$APP_BIN" "\$@"
fi
if command -v deepin-terminal >/dev/null 2>&1; then
  exec deepin-terminal -e "\$APP_BIN" "\$@"
fi
if command -v gnome-terminal >/dev/null 2>&1; then
  exec gnome-terminal -- "\$APP_BIN" "\$@"
fi
if command -v konsole >/dev/null 2>&1; then
  exec konsole -e "\$APP_BIN" "\$@"
fi
if command -v xfce4-terminal >/dev/null 2>&1; then
  exec xfce4-terminal -e "\$APP_BIN" "\$@"
fi
if command -v xterm >/dev/null 2>&1; then
  exec xterm -e "\$APP_BIN" "\$@"
fi

echo "No terminal emulator found. Run directly: \$APP_BIN" >&2
exit 1
EOF
chmod 0755 "$BIN_PKG_DIR/usr/bin/${PKG_NAME}-launcher"

cat > "$BIN_PKG_DIR/usr/share/applications/${PKG_NAME}.desktop" <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Ter Music Rust
Name[zh_CN]=Ter Music Rust
GenericName=Terminal Music Player
Comment=A terminal music player implemented in Rust.
TryExec=/usr/bin/${PKG_NAME}-launcher
Exec=/usr/bin/${PKG_NAME}-launcher
Icon=${PKG_NAME}
Terminal=false
Categories=AudioVideo;Audio;Music;Player;
Keywords=music;player;terminal;rust;
StartupNotify=true
X-Deepin-Vendor=TerMusic
EOF

if [[ -f "$PROJECT_DIR/README.md" ]]; then
  cp "$PROJECT_DIR/README.md" "$BIN_PKG_DIR/usr/share/doc/$PKG_NAME/README.md"
fi
if [[ -f "$PROJECT_DIR/LICENSE" ]]; then
  cp "$PROJECT_DIR/LICENSE" "$BIN_PKG_DIR/usr/share/doc/$PKG_NAME/LICENSE"
fi

cat > "$BIN_PKG_DIR/DEBIAN/control" <<EOF
Package: ${PKG_NAME}
Version: ${PKG_VERSION}
Section: ${PKG_SECTION}
Priority: ${PKG_PRIORITY}
Architecture: ${DEB_ARCH}
Maintainer: ${PKG_MAINTAINER}
Homepage: ${PKG_HOMEPAGE}
Depends: libc6 (>= 2.31), libasound2
Description: A simple and practical terminal-based music player, implemented in Rust, featuring functions such as local/network song search and download, automatic display of lyrics, comment viewing, language and theme switching, and support for Windows, Linux, and MacOS systems.
EOF

cat > "$BIN_PKG_DIR/DEBIAN/postinst" <<EOF
#!/usr/bin/env bash
set -e

if [[ -d /usr/share/deepin/applications ]]; then
  install -m 0644 "/usr/share/applications/${PKG_NAME}.desktop" "/usr/share/deepin/applications/${PKG_NAME}.desktop" || true
fi

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database /usr/share/applications || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -q /usr/share/icons/hicolor || true
fi
exit 0
EOF
chmod 0755 "$BIN_PKG_DIR/DEBIAN/postinst"

cat > "$BIN_PKG_DIR/DEBIAN/postrm" <<EOF
#!/usr/bin/env bash
set -e

if [[ -d /usr/share/deepin/applications ]]; then
  rm -f "/usr/share/deepin/applications/${PKG_NAME}.desktop" || true
fi

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database /usr/share/applications || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -q /usr/share/icons/hicolor || true
fi
exit 0
EOF
chmod 0755 "$BIN_PKG_DIR/DEBIAN/postrm"

if [[ "$WITH_DEBUGINFO" -eq 1 ]]; then
  mkdir -p "$DBG_PKG_DIR/DEBIAN" "$DBG_PKG_DIR/usr/lib/debug/usr/bin"

  objcopy --only-keep-debug "$BIN_PKG_DIR/usr/bin/$PKG_NAME" "$DBG_PKG_DIR/usr/lib/debug/usr/bin/${PKG_NAME}.debug"
  strip --strip-unneeded "$BIN_PKG_DIR/usr/bin/$PKG_NAME"
  objcopy --add-gnu-debuglink="$DBG_PKG_DIR/usr/lib/debug/usr/bin/${PKG_NAME}.debug" "$BIN_PKG_DIR/usr/bin/$PKG_NAME"

  cat > "$DBG_PKG_DIR/DEBIAN/control" <<EOF
Package: ${DBG_PKG_NAME}
Version: ${PKG_VERSION}
Section: debug
Priority: optional
Architecture: ${DEB_ARCH}
Maintainer: ${PKG_MAINTAINER}
Depends: ${PKG_NAME} (= ${PKG_VERSION})
Description: Debug symbols for ${PKG_NAME}
 Detached debug symbols for ${PKG_NAME}.
EOF
fi

DEB_MAIN="$OUTPUT_DIR/${PKG_NAME}_${PKG_VERSION}_${DEB_ARCH}.deb"
dpkg-deb --build "$BIN_PKG_DIR" "$DEB_MAIN"

if [[ "$WITH_DEBUGINFO" -eq 1 ]]; then
  DEB_DBG="$OUTPUT_DIR/${DBG_PKG_NAME}_${PKG_VERSION}_${DEB_ARCH}.deb"
  dpkg-deb --build "$DBG_PKG_DIR" "$DEB_DBG"
fi

if [[ "$WITH_SOURCE" -eq 1 ]]; then
  SOURCE_ROOT="$WORK_DIR/${PKG_NAME}-${PKG_VERSION}"
  DEBIAN_DIR="$SOURCE_ROOT/debian"

  if command -v rsync >/dev/null 2>&1; then
    mkdir -p "$SOURCE_ROOT"
    rsync -a --delete \
      --exclude .git --exclude target --exclude dist --exclude '*.deb' --exclude '*.dsc' --exclude '*.changes' --exclude '*.buildinfo' \
      "$PROJECT_DIR/" "$SOURCE_ROOT/"
  else
    cp -a "$PROJECT_DIR/." "$SOURCE_ROOT/"
    rm -rf "$SOURCE_ROOT/.git" "$SOURCE_ROOT/target" "$SOURCE_ROOT/dist"
  fi

  mkdir -p "$DEBIAN_DIR/source"

  cat > "$DEBIAN_DIR/changelog" <<EOF
${PKG_NAME} (${PKG_VERSION}-1) unstable; urgency=medium

  * Automated source package build.

 -- ${PKG_MAINTAINER}  $(date -R)
EOF

  cat > "$DEBIAN_DIR/control" <<EOF
Source: ${PKG_NAME}
Section: ${PKG_SECTION}
Priority: ${PKG_PRIORITY}
Maintainer: ${PKG_MAINTAINER}
Build-Depends: debhelper-compat (= 13), cargo, rustc, pkg-config, libasound2-dev
Standards-Version: 4.6.2
Rules-Requires-Root: no
Homepage: https://example.invalid/${PKG_NAME}

Package: ${PKG_NAME}
Architecture: any
Depends: \${shlibs:Depends}, \${misc:Depends}, libasound2
Description: A simple and practical terminal-based music player, implemented in Rust, featuring functions such as local/network song search and download, automatic display of lyrics, comment viewing, language and theme switching, and support for Windows, Linux, and MacOS systems.
EOF

  cat > "$DEBIAN_DIR/rules" <<'EOF'
#!/usr/bin/make -f
%:
	dh $@
EOF
  chmod +x "$DEBIAN_DIR/rules"

  cat > "$DEBIAN_DIR/source/format" <<'EOF'
3.0 (quilt)
EOF

  cat > "$DEBIAN_DIR/copyright" <<EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: ${PKG_NAME}
Source: https://example.invalid/${PKG_NAME}

Files: *
Copyright: 2026 Ter Music
License: MIT
EOF

  ORIG_TAR="$WORK_DIR/${PKG_NAME}_${PKG_VERSION}.orig.tar.gz"
  tar -C "$WORK_DIR" --exclude="${PKG_NAME}-${PKG_VERSION}/debian" -czf "$ORIG_TAR" "${PKG_NAME}-${PKG_VERSION}"

  pushd "$WORK_DIR" >/dev/null
  dpkg-source -b "${PKG_NAME}-${PKG_VERSION}"
  popd >/dev/null

  find "$WORK_DIR" -maxdepth 1 -type f \( -name "${PKG_NAME}_${PKG_VERSION}-1.dsc" -o -name "${PKG_NAME}_${PKG_VERSION}.orig.tar.*" -o -name "${PKG_NAME}_${PKG_VERSION}-1.debian.tar.*" \) -exec cp -f {} "$OUTPUT_DIR/" \;
fi

echo "DEB build completed: $OUTPUT_DIR"
find "$OUTPUT_DIR" -maxdepth 1 -type f \( -name '*.deb' -o -name '*.dsc' -o -name '*.tar.*' \) -print
