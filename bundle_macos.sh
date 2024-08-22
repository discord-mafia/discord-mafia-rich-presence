#!/bin/bash

APP_NAME="Discord Mafia Advertisement"
BINARY_NAME="discord-rich-presence-rust"
PROJECT_DIR=$(pwd)
RELEASE_DIR="$PROJECT_DIR/target/release"
APP_DIR="$RELEASE_DIR/$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
PLIST_FILE="$CONTENTS_DIR/Info.plist"
BUNDLE_DIR="$PROJECT_DIR/bundle"

cargo build --release

if [ ! -f "$RELEASE_DIR/$BINARY_NAME" ]; then
    echo "Build failed or binary not found!"
    exit 1
fi

if [ -d "$APP_DIR" ]; then
    rm -rf "$APP_DIR"
fi

mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

cp "$RELEASE_DIR/$BINARY_NAME" "$MACOS_DIR/"

cat <<EOL > "$PLIST_FILE"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.$APP_NAME</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
</dict>
</plist>
EOL

mkdir -p "$BUNDLE_DIR"

mv "$APP_DIR" "$BUNDLE_DIR/"

echo "Successfully bundled $APP_NAME.app in $BUNDLE_DIR"
