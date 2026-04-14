# Rudist

**AI-powered Redis GUI client** built with Rust + egui. Talk to Redis in natural language, keyboard-first design, smooth user experience.

---

## ✨ AI Assistant

Press `Cmd/Ctrl + E` to open the AI panel. Type natural language queries:

```
"Show me all keys matching user:*"
"What's the TTL of session:abc123?"
"Delete expired test keys"
```

The AI translates your questions into Redis commands. Two modes available:

- **Chat Mode** - Safe exploration with command confirmation
- **Agent Mode** - Direct tool execution for power users

👉 See [AI Assistant Overview](ai/index.md) for full capabilities.

---

## ⌨️ Keyboard-First

Minimal mouse, maximum speed:

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + E` | Toggle AI Panel |
| `1-9` | Quick Connect (saved connections) |
| `Cmd/Ctrl + 1-9` | Switch Tabs |
| `Cmd/Ctrl + F` | Focus Filter |

👉 See [Keyboard Shortcuts](usage/shortcuts.md) for complete reference.

---

## Core Features

- **Quick Connect** - `redis://pass@host:port/db` one-line URL
- **Multi-Tab** - Multiple Redis instances simultaneously
- **Full Type Support** - String, List, Hash, Set, ZSet
- **Lazy Loading** - Smooth performance with large datasets
- **Smart JSON** - Auto-format on view, compress on save
- **Live Logs** - Real-time terminal output during AI chat and Redis execution
- **Toast Notifications** - Visual feedback with hover-to-pause

---

## Platforms

| Platform | Format | Architecture |
|----------|--------|--------------|
| **macOS** | `.dmg` / `.app` | Apple Silicon (M1/M2/M3) |
| **Windows** | `.msi` | x64 |

---

## Quick Start

1. Download from [GitHub Releases](https://github.com/davelet/redis-egui-client/releases)
2. Install and launch
3. Create a connection (`Cmd/Ctrl + N`)
4. Press `Cmd/Ctrl + E` → Type: "Show me all keys"

Or build from source:

```bash
git clone https://github.com/davelet/redis-egui-client.git
cd redis-egui-client
cargo run
```

See [Install Guide](install.md) for detailed instructions.

---

## Documentation

| Section | Description |
|---------|-------------|
| [AI Assistant](ai/index.md) | Natural language interface, 18+ tools |
| [Configuration](ai/configuration.md) | Set up models and API keys |
| [Shortcuts](usage/shortcuts.md) | Complete keyboard reference |
| [Usage Guide](usage/index.md) | Full documentation |

---

## License

Apache License 2.0