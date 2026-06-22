#!/usr/bin/env bash
set -euo pipefail

VERSION="${GITHUB_REF_NAME:-0.0.0}"
VERSION="${VERSION#v}"
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+ ]]; then
  VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' crates/app/Cargo.toml | head -n 1)"
fi
PACKAGE_ARCH="${1:-${AKTSQL_PACKAGE_ARCH:-$(uname -m)}}"
case "$PACKAGE_ARCH" in
  arm64 | aarch64)
    PACKAGE_ARCH="arm64"
    ;;
  x86_64 | amd64)
    PACKAGE_ARCH="x64"
    ;;
  *)
    echo "Unsupported macOS architecture: $PACKAGE_ARCH" >&2
    exit 1
    ;;
esac
ROOT="$(pwd)"
APP_DIR="$ROOT/dist/macos/AktSQL.app"
ARTIFACT_DIR="$ROOT/dist/macos-$PACKAGE_ARCH-artifacts"
BIN="$ROOT/target/release/aktsql"

rm -rf "$APP_DIR" "$ARTIFACT_DIR"
mkdir -p "$APP_DIR/Contents/MacOS" "$APP_DIR/Contents/Resources" "$ARTIFACT_DIR"

cp "$BIN" "$APP_DIR/Contents/MacOS/aktsql"
chmod +x "$APP_DIR/Contents/MacOS/aktsql"
cp "$ROOT/crates/ui/assets/aktsql_logo.svg" "$APP_DIR/Contents/Resources/aktsql-logo.svg"

cat > "$APP_DIR/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>AktSQL</string>
  <key>CFBundleDisplayName</key>
  <string>AktSQL</string>
  <key>CFBundleExecutable</key>
  <string>aktsql</string>
  <key>CFBundleIdentifier</key>
  <string>io.aktsql.desktop</string>
  <key>CFBundleVersion</key>
  <string>${VERSION}</string>
  <key>CFBundleShortVersionString</key>
  <string>${VERSION}</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

tar -czf "$ARTIFACT_DIR/AktSQL-macos-$PACKAGE_ARCH-app.tar.gz" -C "$ROOT/dist/macos" "AktSQL.app"
cp "$ARTIFACT_DIR/AktSQL-macos-$PACKAGE_ARCH-app.tar.gz" "$ARTIFACT_DIR/AktSQL-$VERSION-macos-$PACKAGE_ARCH-app.tar.gz"
hdiutil create \
  -volname "AktSQL" \
  -srcfolder "$APP_DIR" \
  -ov \
  -format UDZO \
  "$ARTIFACT_DIR/AktSQL-macos-$PACKAGE_ARCH.dmg"
cp "$ARTIFACT_DIR/AktSQL-macos-$PACKAGE_ARCH.dmg" "$ARTIFACT_DIR/AktSQL-$VERSION-macos-$PACKAGE_ARCH.dmg"
