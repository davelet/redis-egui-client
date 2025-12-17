#!/bin/bash
cd "$(dirname "$0")"
echo "Building release version with optimizations..."
cargo build --release
echo ""
echo "Build complete! Binary location:"
echo "  ./target/release/redis-egui-client"
echo ""
echo "To run:"
echo "  ./target/release/redis-egui-client"
