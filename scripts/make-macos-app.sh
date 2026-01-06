#!/bin/bash

set -e  # 如果任何命令失败，则退出脚本

# 配置
APP_NAME="Rudist"
VERSION="1.0.0"
BUNDLE_ID="com.sheldon.client.redisgui"
BUILD_DIR="target/release"
BINARY_NAME="redis-egui-client"
APP_DIR="$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

# 创建目录结构
echo "Creating directory structure..."
rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR" "$RESOURCES_DIR"

# 复制可执行文件
echo "Copying executable..."
if [ ! -f "$BUILD_DIR/$BINARY_NAME" ]; then
  echo "Binary not found: $BUILD_DIR/$BINARY_NAME"
  echo "Build release binary first: cargo build --release"
  exit 1
fi
cp "$BUILD_DIR/$BINARY_NAME" "$MACOS_DIR/$APP_NAME"
chmod +x "$MACOS_DIR/$APP_NAME"

# 复制资源（如果存在）
if [ -d "assets" ]; then
  echo "Copying assets..."
  cp -r assets/* "$RESOURCES_DIR/" || true
  # If an icon.icns exists, also copy it as AppIcon.icns to match Info.plist
  if [ -f "assets/icon.icns" ]; then
    cp "assets/icon.icns" "$RESOURCES_DIR/AppIcon.icns"
  fi
fi

# 创建 Info.plist
echo "Creating Info.plist..."
cat > "$CONTENTS_DIR/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon.icns</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_ID</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
EOF

# 提示下一步（codesign/notarize）
echo "Application bundle created at $APP_DIR"
echo "Note: To distribute, you should codesign and notarize the bundle using your Apple Developer account."
