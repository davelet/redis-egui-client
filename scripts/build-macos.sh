#!/usr/bin/env bash
set -euo pipefail

# Build macOS .app using cargo-bundle
# Requirements:
#  - cargo-bundle (install with `cargo install cargo-bundle`)
#  - Xcode command line tools
#  - (Optional) codesign identity and notarization credentials for distribution

# Build for current host architecture (recommended for local testing)
cargo bundle --release

# For Apple Silicon (aarch64) explicit target:
# cargo bundle --release --target aarch64-apple-darwin

# For Intel (x86_64) explicit target:
cargo bundle --release --target x86_64-apple-darwin

# After running, look in dist/ for the generated .app and installer packages.

echo "macOS bundles created in ./target/release/bundle/"
open ./target/release/bundle/