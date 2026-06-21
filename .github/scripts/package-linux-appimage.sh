#!/usr/bin/env bash
set -euo pipefail

VERSION="${GITHUB_REF_NAME:-0.0.0}"
VERSION="${VERSION#v}"
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]]; then
  VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' crates/aktsql_app/Cargo.toml | head -n 1)"
fi
ROOT="$(pwd)"
APPDIR="$ROOT/dist/appimage/AktSQL.AppDir"
ARTIFACT_DIR="$ROOT/dist/linux-artifacts"
LINUXDEPLOY="$ROOT/dist/linuxdeploy-x86_64.AppImage"

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
cp "$ROOT/crates/aktsql_app/assets/aktsql_logo.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/aktsql.svg"

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
    https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
  chmod +x "$LINUXDEPLOY"
fi

ARCH=x86_64 "$LINUXDEPLOY" --appdir "$APPDIR" --output appimage
mv "$ROOT"/AktSQL*.AppImage "$ARTIFACT_DIR/AktSQL-linux-x86_64.AppImage"
cp "$ARTIFACT_DIR/AktSQL-linux-x86_64.AppImage" "$ARTIFACT_DIR/AktSQL-$VERSION-linux-x86_64.AppImage"
