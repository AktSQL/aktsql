#!/usr/bin/env bash
set -euo pipefail

VERSION="${GITHUB_REF_NAME:-0.0.0}"
VERSION="${VERSION#v}"
ROOT="$(pwd)"
APP_DIR="$ROOT/dist/macos/AktSQL.app"
ARTIFACT_DIR="$ROOT/dist/macos-artifacts"
BIN="$ROOT/target/release/aktsql"

rm -rf "$APP_DIR" "$ARTIFACT_DIR"
mkdir -p "$APP_DIR/Contents/MacOS" "$APP_DIR/Contents/Resources" "$ARTIFACT_DIR"

cp "$BIN" "$APP_DIR/Contents/MacOS/aktsql"
chmod +x "$APP_DIR/Contents/MacOS/aktsql"
cp "$ROOT/crates/aktsql_app/assets/aktsql_logo.svg" "$APP_DIR/Contents/Resources/aktsql-logo.svg"

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

tar -czf "$ARTIFACT_DIR/AktSQL-$VERSION-macos-app.tar.gz" -C "$ROOT/dist/macos" "AktSQL.app"
hdiutil create \
  -volname "AktSQL" \
  -srcfolder "$APP_DIR" \
  -ov \
  -format UDZO \
  "$ARTIFACT_DIR/AktSQL-$VERSION-macos.dmg"
