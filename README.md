# Rudist - Redis GUI Client

[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml)
[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml)
[![Version](https://img.shields.io/github/v/release/davelet/redis-egui-client?label=version&color=blue)](https://github.com/davelet/redis-egui-client/releases)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)](LICENSE)

[English](README.md) | [中文](README.zh-CN.md)

A modern, high-performance Redis GUI client built with Rust and egui, supporting macOS and Windows.

## Core Features

### Multi-Connection Management
- **Multi-Tab Support** - Open multiple Redis connections simultaneously, switch between tabs quickly
- **Connection Color Coding** - Assign different colors to different environments (dev/test/prod) for instant visual distinction
- **Persistent Configuration** - Connection settings are automatically saved and available on next launch
- **Connect All** - Quickly restore all previously opened connections
- **Quick Connect** - Connect via Redis URL (`redis://user:pass@host:port/db`) or `rediss://` for TLS
- **TLS Support** - Secure TLS/SSL connections in both Quick Connect and Advanced modes

### Data Browsing & Editing
- **Full Type Support** - Complete support for String, List, Hash, Set, ZSet
- **Smart JSON Handling** - Auto-format JSON data for friendly editing, intelligent compression on save
- **Lazy Loading** - Smooth browsing with large datasets, load keys and values on demand
- **Real-time Filtering** - Key list supports real-time search and filtering
- **Group Keys by Colon** - Organize keys in a tree structure based on colon separator (e.g., `user:123:profile`)

### Efficient Operations
- **Keyboard-First** - Shortcut support reduces mouse usage and improves efficiency
- **Command Line Mode** - Execute any Redis command directly
- **TTL Management** - Conveniently view and modify key expiration times
- **Data Operations** - Support copy, delete, rename, and other common operations

### Performance Optimization
- **Async I/O** - Tokio-based asynchronous architecture, operations never block
- **Incremental Loading** - Batch loading for large key sets, UI remains responsive
- **Memory Optimization** - Large values loaded on demand, prevents memory spikes

### Cross-Platform Support
- **macOS** - Native App Bundle (.app) with Apple Silicon (M1/M2/M3) support; Intel Mac supported via [source build](#build-from-source)
- **Windows** - MSI installer with application icon and system PATH integration

### AI-Powered Assistant
- **Natural Language Interface** - Control Redis using plain English commands
- **Tool-Enabled AI** - AI can execute 18+ operations including:
  - Query: filter keys, get key info, check existence, database stats
  - Write: set values, set TTL, rename/delete keys
  - Data Structures: Hash (hset/hdel), List (lset/rpush), Set (sadd/srem), ZSet (zadd/zrem)
  - Database: execute commands, switch databases
- **Multi-Round Conversation** - Maintains context for complex multi-step tasks
- **Flexible Model Support** - Works with OpenAI, Claude, Ollama, OpenRouter, and any OpenAI-compatible API

## Quick Start

### Installation

Download the latest release from [GitHub Releases](https://github.com/davelet/redis-egui-client/releases).

#### macOS
Download the `.dmg` file and drag the app to your Applications folder.

#### Windows
Download and run the `.msi` installer.

### Build from Source

```bash
# Clone the repository
git clone https://github.com/davelet/redis-egui-client.git
cd redis-egui-client

# Run in development mode
cargo run

# Release build
cargo build --release
```

### macOS App Bundle Build

```bash
# Install cargo-bundle
cargo install cargo-bundle

# Build macOS app
./scripts/build-macos.sh
```

### Windows Build (with icon support)

The Windows build automatically includes application icon generation. Ensure you have the necessary Rust toolchain for Windows targets:

```bash
# Build for Windows (from any platform if cross-compilation is set up)
cargo build --release --target x86_64-pc-windows-msvc
```

## Usage Guide

### Creating a Connection
1. Click the dropdown at the top and select "+ New"
2. Fill in connection details (name, host, port, password, etc.)
3. Choose a connection color to distinguish environments
4. Click save, connection config will be persisted locally

### Managing Data
- **Browse Keys** - Left panel shows all keys, supports `*` wildcard filtering
- **View Values** - Click a key to see detailed content on the right
- **Edit Data** - Double-click a value or click edit button, JSON data auto-formats
- **Execute Commands** - Enter any Redis command in the bottom command line, press Enter to execute

### Keyboard Shortcuts
| Shortcut | Function |
|----------|----------|
| `Cmd/Ctrl + T` | New Tab |
| `Cmd/Ctrl + W` | Close current Tab |
| `Cmd/Ctrl + R` | Refresh current Key |
| `Cmd/Ctrl + F` | Focus to Key filter box |
| `Cmd/Ctrl + 1-9` | Switch to Tab 1-9 |
| `Cmd/Ctrl + 0` | Switch to the rightmost Tab |
| `Cmd/Ctrl + E` | Toggle Command Line Panel |
| `Up/Down Arrow` | Navigate command history (in command line) |
| `Cmd/Ctrl + Shift + D` | Delete duplicate Tabs and unconnected Tabs |

## Configuration

Configuration file locations:
- **macOS**: `~/.config/rudist/config.toml`
- **Windows**: `%APPDATA%\rudist\config.toml`

Example configuration:
```toml
[[connections]]
name = "Local Dev"
host = "localhost"
port = 6379
password = ""
database = 0
color = "#4CAF50"

[[connections]]
name = "Production"
host = "redis.example.com"
port = 6380
password = "secret"
database = 0
color = "#F44336"
```

## Contributing

Issues and PRs are welcome!

```bash
# Setup Git hooks
sh scripts/setup-git-hooks.sh

# Code formatting
cargo fmt --all

# Run tests
cargo test
```

## License

Apache License 2.0 - See [LICENSE](LICENSE) file for details

## Author

Sheldon.Wei <sheldon.sh.hb@gmail.com>

---

⭐ If this project helps you, please give it a Star!
