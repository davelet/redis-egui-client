# Rudist - Redis GUI Client with AI

[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml)
[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml)
[![Version](https://img.shields.io/github/v/release/davelet/redis-egui-client?label=version&color=blue)](https://github.com/davelet/redis-egui-client/releases)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)](LICENSE)

[English](README.md) | [中文](README.zh-CN.md)

A modern Redis GUI client with built-in AI assistant. Manage Redis using natural language, keyboard-first design, and smooth user experience.

---

## ✨ AI-Powered Assistant

**Talk to Redis in plain English.** No need to memorize commands.

```
You: "Show me all keys matching user:*"
AI:  → SCAN 0 MATCH user:* COUNT 100

You: "What's the TTL of session:abc123?"
AI:  → TTL session:abc123

You: "Delete all expired test keys"
AI:  → Analyzes and suggests cleanup commands
```

### Two AI Modes

| Mode | Description | Best For |
|------|-------------|----------|
| **Chat Mode** | Translates natural language → Redis command (requires confirmation) | Safe exploration, learning Redis |
| **Agent Mode** | Full tool access, executes 18+ operations directly | Power users, complex workflows |

### AI Capabilities (18+ Tools)

- **Query**: filter keys, get key info, check existence, database stats
- **Write**: set values, set TTL, rename/delete keys
- **Data Structures**: Hash (hset/hdel), List (lset/rpush), Set (sadd/srem), ZSet (zadd/zrem)
- **Database**: execute commands, switch databases

### Flexible Model Support

Works with OpenAI, Claude, Ollama (local), OpenRouter, and any OpenAI-compatible API. Your API keys are stored securely in system keychain (never in config files).

👉 **Quick Start**: Press `Cmd/Ctrl + E` to open AI panel, type your question.

---

## ⌨️ Keyboard-First Design

**Minimal mouse, maximum speed.** Every action has a shortcut.

### Essential Shortcuts

| `Cmd/Ctrl + E` | **Toggle AI Panel** ← Start here |
| `Cmd/Ctrl + L` | **Toggle Live Logs** |
| `Cmd/Ctrl + T` | New Tab |
| `Cmd/Ctrl + W` | Close Tab |
| `Cmd/Ctrl + R` | Refresh Key |
| `Cmd/Ctrl + F` | Focus Filter |
| `1-9` | Quick Connect (saved connections) |
| `Cmd/Ctrl + 1-9` | Switch Tabs |
| `↑/↓` | Command history navigation |
| `Esc` | Close panel/dialog |

All shortcuts are customizable in Settings (`Cmd/Ctrl + ,`).

---

## 🎯 Smooth User Experience

- **Live Logs** - Real-time terminal output during AI chat and Redis execution
- **Toast Notifications** - Connection errors, warnings with hover-to-pause
- **Multi-Tab Workflow** - Independent sessions, color-coded environments
- **Lazy Loading** - Large datasets load smoothly, UI never freezes
- **Smart JSON** - Auto-format on view, compress on save
- **Progress Indicators** - Visual feedback for all async operations

---

## Core Features

### Connection Management
- **Quick Connect** - One-line URL: `redis://pass@host:port/db` or `rediss://` for TLS
- **Multi-Tab** - Multiple Redis instances simultaneously
- **Color Coding** - Visual distinction for dev/test/prod environments
- **Persistent Config** - Auto-saved, restored on launch

### Data Operations
- **Full Type Support** - String, List, Hash, Set, ZSet
- **Key Tree View** - Organize by colon separator (`user:123:profile`)
- **Real-time Filter** - Pattern matching with `*` wildcard
- **TTL Management** - View and modify expiration times
- **CRUD Operations** - Create, edit, delete keys with confirmation

### Performance
- **Async I/O** - Tokio-based, never blocks UI
- **Incremental Loading** - Batch loading for millions of keys
- **Memory Efficient** - Large values loaded on demand

### Cross-Platform
- **macOS** - Native .app, Apple Silicon (M1/M2/M3) support
- **Windows** - MSI installer with PATH integration

---

## Quick Start

### Install

Download from [GitHub Releases](https://github.com/davelet/redis-egui-client/releases):

- **macOS**: `.dmg` → drag to Applications
- **Windows**: `.msi` → run installer

### Build from Source

```bash
git clone https://github.com/davelet/redis-egui-client.git
cd redis-egui-client
cargo run
```

### First Run

1. Press `Cmd/Ctrl + N` to create a connection
2. Enter host/port (or use Quick Connect URL)
3. Press `Cmd/Ctrl + E` to open AI panel
4. Type: "Show me all keys" ← Start exploring!

---

## Documentation

- [AI Configuration](docs/ai/configuration.md) - Set up models and API keys
- [AI Tools Reference](docs/ai/tools.md) - Full list of 18+ tools
- [Keyboard Shortcuts](docs/usage/shortcuts.md) - Complete shortcut reference
- [Usage Guide](docs/usage/index.md) - Full documentation

---

## Contributing

```bash
cargo fmt --all
cargo test
```

## License

Apache License 2.0

## Author

Sheldon.Wei <sheldon.sh.hb@gmail.com>

---

⭐ If this project helps you, please give it a Star!