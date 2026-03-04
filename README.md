# Rudist - Redis GUI Client

[![Version](https://img.shields.io/badge/version-0.3.0-blue.svg)](https://github.com/davelet/redis-egui-client)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)](LICENSE)

[English](README.md) | [中文](README.zh-CN.md)

A modern, high-performance Redis GUI client built with Rust and egui, supporting cross-platform (macOS, Windows, Linux).

## Core Features

### Multi-Connection Management
- **Multi-Tab Support** - Open multiple Redis connections simultaneously, switch between tabs quickly
- **Connection Color Coding** - Assign different colors to different environments (dev/test/prod) for instant visual distinction
- **Persistent Configuration** - Connection settings are automatically saved and available on next launch
- **Connect All** - Quickly restore all previously opened connections

### Data Browsing & Editing
- **Full Type Support** - Complete support for String, List, Hash, Set, ZSet
- **Smart JSON Handling** - Auto-format JSON data for friendly editing, intelligent compression on save
- **Lazy Loading** - Smooth browsing with large datasets, load keys and values on demand
- **Real-time Filtering** - Key list supports real-time search and filtering

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
- **macOS** - Native App Bundle support, both Apple Silicon and Intel
- **Windows** - MSI installer with system PATH integration
- **Linux** - Standalone executable

## UI Preview

```
┌─────────────────────────────────────────────────────────────┐
│ [Tab 1] [Tab 2] [+ Tab]                    [Connect] [Settings]│
├──────────┬──────────────────────────────────────────────────┤
│          │  Key: user:12345                    [Save] [Delete]│
│  Keys    │  TTL: 3600s                                       │
│  ─────── │                                                   │
│  user:*  │  Type: Hash                                       │
│  session:│  ┌─────────┬─────────────────────────────────────┐│
│  cache:  │  │  field  │  value                              ││
│          │  ├─────────┼─────────────────────────────────────┤│
│          │  │  name   │  "John Doe"                         ││
│          │  │  email  │  {                                  ││
│          │  │         │    "type": "work",                  ││
│          │  │         │    "address": "john@example.com"    ││
│          │  │         │  }                                  ││
│          │  └─────────┴─────────────────────────────────────┘│
│          │                                                   │
│          │  > EXECUTE REDIS COMMAND                          │
└──────────┴──────────────────────────────────────────────────┘
```

## Quick Start

### Installation

#### macOS
```bash
# Using Homebrew (coming soon)
brew install rudist

# Or download the dmg installer
```

#### Windows
Download and run the `.msi` installer.

#### Linux
```bash
cargo install rudist
```

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
| `Enter` | Execute command |

## Configuration

Configuration file locations:
- **macOS/Linux**: `~/.config/rudist/config.toml`
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

## Tech Stack

- **Rust 2024 Edition** - Systems-level performance, memory safety
- **egui + eframe** - Immediate mode GUI, smooth response
- **Tokio** - Async runtime, efficient I/O
- **redis-rs** - Redis client library
- **serde_json** - JSON data processing

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
