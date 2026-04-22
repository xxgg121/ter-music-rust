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
command -v rpmbuild >/dev/null 2>&1 || { echo "Error: rpmbuild not found" >&2; exit 1; }
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
URL:            https://example.invalid/${PKG_NAME}
Source0:        %{name}-%{version}.tar.gz
BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  gcc
BuildRequires:  binutils
Requires:       alsa-lib

%description
A terminal music player implemented in Rust.

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
install -m 0755 target/${TARGET}/release/${PKG_NAME} %{buildroot}%{_bindir}/${PKG_NAME}

%files
%license LICENSE*
%doc README.md
%{_bindir}/${PKG_NAME}

%changelog
* Wed Apr 22 2026 CodeBuddy <noreply@example.invalid> - ${PKG_VERSION}-${PKG_RELEASE}
- Automated RPM build script output
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
