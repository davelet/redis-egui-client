## Overview

Rudist is a modern Redis GUI client built with Rust + egui.

### Key Features

- **Multi-tab connections** - Manage multiple Redis connections simultaneously
- **Full data type support** - String, List, Hash, Set, ZSet
- **JSON auto-formatting** - Smart JSON parsing and display
- **Keyboard shortcuts** - Efficient mouse-free operation
- **Async I/O** - Fast response, high performance
- **AI assistance** - Natural language to Redis commands

### Platform Support

- macOS (Apple Silicon / Intel)
- Windows

---

## Connection

### New Connection

Click the "New Connection" button in the top toolbar, or use shortcut `Cmd+N` (macOS) or `Ctrl+N` (Windows).

### Connection Parameters

| Parameter | Description |
|-----------|-------------|
| Connection Name | Identifier for easy reference |
| Address | Redis server address (IP or hostname) |
| Port | Default 6379 |
| Password | Fill if required |
| Database | Database index after connecting (0-15) |

### Auto Connect

When the app starts, it will prompt to restore unclosed connections from the previous session. You can enable or disable this prompt in Settings.

---

## Keys

### Filter Keys

Enter keywords in the filter box above the key list and press `Enter`. Supports wildcard `*`.

### Refresh Key List

Press `Cmd+R` (macOS) or `Ctrl+R` (Windows) to reload all keys.

### New Key

Right-click the key list and select "New Key", or use shortcut `Cmd+Shift+N`.

### Delete Key

Select a key and press `Delete`, or right-click and select "Delete".

### Copy Key Name

Select a key and click the copy button, or right-click and select "Copy Key".

---

## Value Viewer

### Supported Data Types

| Type | Display | Operations |
|------|---------|------------|
| **String** | Plain text | Edit, Copy |
| **List** | Indexed items | Add, remove, edit elements |
| **Hash** | Field-value table | Add, remove, edit fields |
| **Set** | Member list | Add, remove members |
| **ZSet** | Scored members | Add, remove, edit scores |

### JSON Formatting

Automatically detects and formats JSON data with syntax highlighting.

### TTL Operations

- View remaining TTL
- Modify TTL value
- Remove expiration

---

## Command Line

### Open Command Line

Press `Cmd+E` (macOS) or `Ctrl+E` (Windows) to open the command line panel.

### Modes

- **Normal mode** - Enter Redis commands directly
- **AI mode** - Describe operations in natural language, AI converts to Redis commands

### Examples

```
# Normal command
GET mykey

# AI mode
show all keys containing "user"
```

---

## Shortcuts

Keyboard shortcuts help you work efficiently without using the mouse.

All shortcuts can be customized in Settings. Press the button below to open the keyboard shortcuts settings.

[View All Shortcuts in Settings]

---

## Settings

### Theme

- **Light** - White background
- **Dark** - Dark background, suitable for nighttime

### Language

- English
- Chinese

### Keyboard Customization

All customizable shortcuts can be modified in Settings. Click the shortcut input and press the new key combination.

### AI Configuration

Add API Key and select a model in Settings for AI-assisted features.

---

## Online Docs

For more details and advanced usage, visit our online documentation.