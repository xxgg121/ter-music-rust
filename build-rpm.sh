#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: ./build-rpm.sh [options]

Build RPM package for ter-music-rust.

Options:
  --with-debuginfo     Build with debug info enabled and emit debuginfo package
  --target <triple>    Rust target triple (default: x86_64-unknown-linux-gnu)
  --release-dir <dir>  Output directory for generated RPMs (default: ./dist/rpm)
  -h, --help           Show this help message
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"

WITH_DEBUGINFO=0
TARGET="x86_64-unknown-linux-gnu"
OUTPUT_DIR="$PROJECT_DIR/dist/rpm"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-debuginfo)
      WITH_DEBUGINFO=1
      shift
      ;;
    --target)
      TARGET="${2:-}"
      [[ -n "$TARGET" ]] || { echo "Error: --target requires a value" >&2; exit 1; }
      shift 2
      ;;
    --release-dir)
      OUTPUT_DIR="${2:-}"
      [[ -n "$OUTPUT_DIR" ]] || { echo "Error: --release-dir requires a value" >&2; exit 1; }
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
if ! command -v rpmbuild >/dev/null 2>&1; then
  echo "Error: rpmbuild not found" >&2
  if command -v apt-get >/dev/null 2>&1; then
    echo "Hint (Debian/Deepin/Ubuntu): sudo apt-get update && sudo apt-get install -y rpm" >&2
  elif command -v dnf >/dev/null 2>&1; then
    echo "Hint (Fedora/RHEL): sudo dnf install -y rpm-build" >&2
  elif command -v yum >/dev/null 2>&1; then
    echo "Hint (CentOS/RHEL): sudo yum install -y rpm-build" >&2
  elif command -v zypper >/dev/null 2>&1; then
    echo "Hint (openSUSE): sudo zypper install -y rpm-build" >&2
  fi
  exit 1
fi
command -v python3 >/dev/null 2>&1 || { echo "Error: python3 not found" >&2; exit 1; }

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
PKG_RELEASE="1"

TOPDIR="$PROJECT_DIR/target/rpmbuild"
SOURCES_DIR="$TOPDIR/SOURCES"
SPECS_DIR="$TOPDIR/SPECS"
rm -rf "$TOPDIR"
mkdir -p "$SOURCES_DIR" "$SPECS_DIR" "$OUTPUT_DIR"

STAGING="$TOPDIR/${PKG_NAME}-${PKG_VERSION}"
mkdir -p "$STAGING"

if command -v rsync >/dev/null 2>&1; then
  rsync -a --delete \
    --exclude .git --exclude target --exclude dist --exclude '*.rpm' --exclude '*.deb' \
    "$PROJECT_DIR/" "$STAGING/"
else
  cp -a "$PROJECT_DIR/." "$STAGING/"
  rm -rf "$STAGING/.git" "$STAGING/target" "$STAGING/dist"
fi

TARBALL="$SOURCES_DIR/${PKG_NAME}-${PKG_VERSION}.tar.gz"
tar -C "$TOPDIR" -czf "$TARBALL" "${PKG_NAME}-${PKG_VERSION}"

SPEC_FILE="$SPECS_DIR/${PKG_NAME}.spec"
cat > "$SPEC_FILE" <<EOF
%global _build_id_links none
%if ! 0%{?with_debuginfo:1}
%global debug_package %{nil}
%endif

Name:           ${PKG_NAME}
Version:        ${PKG_VERSION}
Release:        ${PKG_RELEASE}%{?dist}
Summary:        Terminal music player written in Rust
License:        MIT
URL:            https://github.com/xxgg121/ter-music-rust
Source0:        %{name}-%{version}.tar.gz
BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  gcc
BuildRequires:  binutils
Requires:       alsa-lib

%description
A simple and practical terminal-based music player, implemented in Rust, featuring functions such as local/network song search and download, automatic display of lyrics, comment viewing, language and theme switching, and support for Windows, Linux, and MacOS systems.

%prep
%autosetup

%build
%if 0%{?with_debuginfo:1}
export RUSTFLAGS="-C debuginfo=2"
export CARGO_PROFILE_RELEASE_STRIP=none
cargo build --release --target ${TARGET}
%else
cargo build --release --target ${TARGET}
%endif

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/pixmaps
for sz in 96 128 256 512; do
  mkdir -p "%{buildroot}%{_datadir}/icons/hicolor/${sz}x${sz}/apps"
done

install -m 0755 target/${TARGET}/release/${PKG_NAME} %{buildroot}%{_bindir}/${PKG_NAME}

cat > %{buildroot}%{_bindir}/${PKG_NAME}-launcher <<'LAUNCHER_EOF'
#!/usr/bin/env bash
set -e
APP_BIN="/usr/bin/${PKG_NAME}"

if command -v x-terminal-emulator >/dev/null 2>&1; then
  exec x-terminal-emulator -e "$APP_BIN" "$@"
fi
if command -v deepin-terminal >/dev/null 2>&1; then
  exec deepin-terminal -e "$APP_BIN" "$@"
fi
if command -v gnome-terminal >/dev/null 2>&1; then
  exec gnome-terminal -- "$APP_BIN" "$@"
fi
if command -v konsole >/dev/null 2>&1; then
  exec konsole -e "$APP_BIN" "$@"
fi
if command -v xfce4-terminal >/dev/null 2>&1; then
  exec xfce4-terminal -e "$APP_BIN" "$@"
fi
if command -v xterm >/dev/null 2>&1; then
  exec xterm -e "$APP_BIN" "$@"
fi

echo "No terminal emulator found. Run directly: $APP_BIN" >&2
exit 1
LAUNCHER_EOF
chmod 0755 %{buildroot}%{_bindir}/${PKG_NAME}-launcher

for sz in 96 128 256 512; do
  icon_file="assets/icons/ter-music-rust-${sz}.png"
  if [ -f "$icon_file" ]; then
    install -m 0644 "$icon_file" "%{buildroot}%{_datadir}/icons/hicolor/${sz}x${sz}/apps/${PKG_NAME}.png"
  fi
done

if [ -f assets/icons/ter-music-rust.png ]; then
  install -m 0644 assets/icons/ter-music-rust.png %{buildroot}%{_datadir}/pixmaps/${PKG_NAME}.png
elif [ -f assets/icons/ter-music-rust-512.png ]; then
  install -m 0644 assets/icons/ter-music-rust-512.png %{buildroot}%{_datadir}/pixmaps/${PKG_NAME}.png
fi

cat > %{buildroot}%{_datadir}/applications/${PKG_NAME}.desktop <<DESKTOP_EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Ter Music Rust
Name[zh_CN]=Ter Music Rust
GenericName=Terminal Music Player
Comment=A simple and practical terminal-based music player, implemented in Rust, featuring functions such as local/network song search and download, automatic display of lyrics, comment viewing, language and theme switching, and support for Windows, Linux, and MacOS systems.
TryExec=/usr/bin/${PKG_NAME}-launcher
Exec=/usr/bin/${PKG_NAME}-launcher
Icon=${PKG_NAME}
Terminal=false
Categories=AudioVideo;Audio;Music;Player;
Keywords=music;player;terminal;rust;
StartupNotify=true
X-Deepin-Vendor=TerMusic
DESKTOP_EOF

%post
if [ -x /usr/bin/update-desktop-database ]; then
  /usr/bin/update-desktop-database -q %{_datadir}/applications || :
fi
if [ -x /usr/bin/gtk-update-icon-cache ]; then
  /usr/bin/gtk-update-icon-cache -q %{_datadir}/icons/hicolor || :
fi

%postun
if [ -x /usr/bin/update-desktop-database ]; then
  /usr/bin/update-desktop-database -q %{_datadir}/applications || :
fi
if [ -x /usr/bin/gtk-update-icon-cache ]; then
  /usr/bin/gtk-update-icon-cache -q %{_datadir}/icons/hicolor || :
fi

%files
%license LICENSE*
%doc README.md
%{_bindir}/${PKG_NAME}
%{_bindir}/${PKG_NAME}-launcher
%{_datadir}/applications/${PKG_NAME}.desktop
%{_datadir}/pixmaps/${PKG_NAME}.png
%{_datadir}/icons/hicolor/96x96/apps/${PKG_NAME}.png
%{_datadir}/icons/hicolor/128x128/apps/${PKG_NAME}.png
%{_datadir}/icons/hicolor/256x256/apps/${PKG_NAME}.png
%{_datadir}/icons/hicolor/512x512/apps/${PKG_NAME}.png

%changelog
* Wed Apr 22 2026 CodeBuddy <noreply@example.invalid> - ${PKG_VERSION}-${PKG_RELEASE}
- Add desktop launcher and icons for GUI startup
EOF

RPM_OPTS=(
  --define "_topdir $TOPDIR"
  --define "_rpmdir $OUTPUT_DIR"
  --define "_srcrpmdir $OUTPUT_DIR"
)

if [[ "$WITH_DEBUGINFO" -eq 1 ]]; then
  RPM_OPTS+=(--with debuginfo)
else
  RPM_OPTS+=(--without debuginfo)
fi

rpmbuild -ba "${RPM_OPTS[@]}" "$SPEC_FILE"

echo "RPM build completed: $OUTPUT_DIR"
find "$OUTPUT_DIR" -type f \( -name '*.rpm' -o -name '*.src.rpm' \) -print
