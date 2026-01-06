#!/usr/bin/env bash
set -euo pipefail

# Generate macOS .icns from assets/icon.svg
# Requires: iconutil (macOS), sips (macOS), and either rsvg-convert or ImageMagick's convert

SVG="assets/icon.svg"
PNG="assets/icon-1024.png"
ICONSET="assets/icon.iconset"
ICNS="assets/icon.icns"

if [ ! -f "$SVG" ]; then
  echo "SVG source not found at $SVG"
  exit 1
fi

# Rasterize SVG to 1024x1024 PNG
if command -v rsvg-convert >/dev/null 2>&1; then
  echo "Using rsvg-convert to rasterize SVG..."
  rsvg-convert -w 1024 -h 1024 -o "$PNG" "$SVG"
elif command -v convert >/dev/null 2>&1; then
  echo "Using ImageMagick convert to rasterize SVG..."
  convert -background none -resize 1024x1024 "$SVG" "$PNG"
else
  echo "No SVG rasterizer found (rsvg-convert or convert). Please install librsvg or ImageMagick, or provide assets/icon-1024.png manually."
  exit 1
fi

# Create iconset
rm -rf "$ICONSET"
mkdir -p "$ICONSET"

# sizes required by iconutil
sizes=(16 32 64 128 256 512)
for size in "${sizes[@]}"; do
  sips -z $size $size "$PNG" --out "$ICONSET/icon_${size}x${size}.png" >/dev/null
  sips -z $(($size*2)) $(($size*2)) "$PNG" --out "$ICONSET/icon_${size}x${size}@2x.png" >/dev/null
done

# also put 1024x1024 as icon_512x512@2x
cp "$PNG" "$ICONSET/icon_512x512@2x.png"

# Build .icns
if command -v iconutil >/dev/null 2>&1; then
  iconutil -c icns "$ICONSET" -o "$ICNS"
  echo "Generated $ICNS"
else
  echo "iconutil not found. On macOS install Xcode command line tools. The iconset is available at $ICONSET"
fi

# Cleanup intermediate PNG if desired
# rm -f "$PNG"

echo "Done."