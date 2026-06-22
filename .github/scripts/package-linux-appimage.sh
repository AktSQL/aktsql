#!/usr/bin/env bash
set -euo pipefail

VERSION="${GITHUB_REF_NAME:-0.0.0}"
VERSION="${VERSION#v}"
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]]; then
  VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' crates/app/Cargo.toml | head -n 1)"
fi
PACKAGE_ARCH="${1:-${AKTSQL_PACKAGE_ARCH:-$(uname -m)}}"
case "$PACKAGE_ARCH" in
  x86_64 | amd64)
    PACKAGE_ARCH="x86_64"
    ;;
  aarch64 | arm64)
    PACKAGE_ARCH="aarch64"
    ;;
  *)
    echo "Unsupported AppImage architecture: $PACKAGE_ARCH" >&2
    exit 1
    ;;
esac
ROOT="$(pwd)"
APPDIR="$ROOT/dist/appimage-$PACKAGE_ARCH/AktSQL.AppDir"
ARTIFACT_DIR="$ROOT/dist/linux-$PACKAGE_ARCH-artifacts"
LINUXDEPLOY="$ROOT/dist/linuxdeploy-$PACKAGE_ARCH.AppImage"

rm -rf "$APPDIR" "$ARTIFACT_DIR"
mkdir -p \
  "$APPDIR/usr/bin" \
  "$APPDIR/usr/share/applications" \
  "$APPDIR/usr/share/icons/hicolor/scalable/apps" \
  "$ARTIFACT_DIR" \
  "$ROOT/dist"

cp "$ROOT/target/release/aktsql" "$APPDIR/usr/bin/aktsql"
chmod +x "$APPDIR/usr/bin/aktsql"
cp "$ROOT/packaging/linux/aktsql.desktop" "$APPDIR/usr/share/applications/aktsql.desktop"
cp "$ROOT/crates/ui/assets/aktsql_logo.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/aktsql.svg"

cat > "$APPDIR/AppRun" <<'APPRUN'
#!/usr/bin/env bash
SELF="$(readlink -f "$0")"
HERE="${SELF%/*}"
exec "$HERE/usr/bin/aktsql" "$@"
APPRUN
chmod +x "$APPDIR/AppRun"

if [ ! -f "$LINUXDEPLOY" ]; then
  curl -L \
    -o "$LINUXDEPLOY" \
    "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-$PACKAGE_ARCH.AppImage"
  chmod +x "$LINUXDEPLOY"
fi

ARCH="$PACKAGE_ARCH" "$LINUXDEPLOY" --appdir "$APPDIR" --output appimage
mv "$ROOT"/AktSQL*.AppImage "$ARTIFACT_DIR/AktSQL-linux-$PACKAGE_ARCH.AppImage"
cp "$ARTIFACT_DIR/AktSQL-linux-$PACKAGE_ARCH.AppImage" "$ARTIFACT_DIR/AktSQL-$VERSION-linux-$PACKAGE_ARCH.AppImage"
