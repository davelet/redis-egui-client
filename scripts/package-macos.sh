#!/bin/bash
set -e

echo "Building release..."
cargo build --release -p e-client-app

echo "Packaging app and dmg..."
cargo packager --release --formats app,dmg --manifest-path e-client-app/Cargo.toml --out-dir dist --binaries-dir target/release

echo "Done! Output files:"
ls -lh dist/*.app dist/*.dmg 2>/dev/null || echo "No files found"
