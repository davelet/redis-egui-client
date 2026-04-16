## Overview

Rudist is a modern Redis GUI client built with Rust + egui.

### Key Features

- **Multi-tab connections** - Manage multiple Redis connections simultaneously
- **Full data type support** - String, List, Hash, Set, ZSet
- **JSON auto-formatting** - Smart JSON parsing and display
- **Keyboard shortcuts** - Efficient mouse-free operation
- **Async I/O** - Fast response, high performance
- **AI assistance** - Natural language to Redis commands (Chat & Agent modes)

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
- **AI Chat mode** - Stateless: translate natural language to ONE Redis command
- **AI Agent mode** - Stateful: use tools to interact with Redis (view, edit, delete keys, etc.)

### AI Chat Mode

Single command translation. Example:
```
# What keys do I have?
KEYS *
```

### AI Agent Mode

Multi-turn conversation with Redis tools:
- View/search keys, view key details (type, TTL, value)
- Edit key values, set TTL, rename keys
- Delete keys
- Execute any Redis command
- Switch databases
- Manage Hash, List, Set, ZSet data structures

Example:
```
# Show me all user keys
→ filter_keys("user:*")

# What's in session:123?
→ get_key_info("session:123")

→ filter_keys("cache:*") → delete_keys([...])
```

### Interrupting AI

If an AI request is taking too long or an Agent is performing unintended operations:
- Click the red square button (**🟥**) appearing next to the status.
- This will immediately abort the request and stop any background processes.
- **Note:** No keyboard shortcut is provided to prevent accidental interruption.

---

## Shortcuts

Keyboard shortcuts help you work efficiently without using the mouse.

All shortcuts can be customized in Settings. Press the button below to open the keyboard shortcuts settings.

[View All Shortcuts in Settings]

---

## Settings

### Theme

Choose from 11 built-in themes:

| Theme | Description |
|-------|-------------|
| **System** | Follow system dark/light preference |
| **Light** | White background |
| **Dark** | Dark background |
| **Dracula** | Purple accent theme |
| **Nord** | Nordic cold gray-blue |
| **Gruvbox** | Retro warm colors |
| **Monokai** | Programming syntax highlighting style |
| **One Dark** | Atom editor style |
| **Tokyo Night** | Tokyo night cityscape |
| **Solarized Dark** | Solarized dark variant |
| **Solarized Light** | Solarized light variant |

Theme changes apply immediately.

### Global Font

- **Global Monospace** - Apply monospace font to all text (otherwise only for data viewing)

### Language

- English
- Chinese (中文)

### Connection Settings

- **Open connections in new tab** - When enabled, new connections open in a separate tab
- **Auto connect** - Automatically connect to the last used connection on startup
- **Show unclosed connections** - Prompt to restore connections from previous session
- **Allow duplicate connections** - Allow connecting to the same Redis server multiple times

### Display Settings

- **Group keys by colon** - Organize keys hierarchically using `:` as separator (e.g., `user:123:name`)
- **Auto refresh TTL** - Automatically refresh TTL display for keys
- **Auto expand composite types** - Automatically expand Hash, List, Set, ZSet when member count is below threshold

### Update Settings

The app can automatically check for new versions from GitHub Releases.

- **Check for updates on startup** - Automatically check for new versions when the app starts
- **Check interval** - Set the frequency for automatic checks (Daily / Weekly / Monthly)
- **Manual check** - Click "Check for Updates" to check immediately

When a new version is available:

- A **green version number** (e.g., `v1.2.0`) appears next to the settings button in the top toolbar. Click it to open the specific Release page for download.
- The Update section in the Settings panel shows the new version info and action buttons.

**Skip version:** If you don't want to upgrade to a particular version, click "Skip this version" and you won't be reminded again. You can view skipped versions in the Settings panel and click ✖ to cancel the skip.

### Keyboard Customization

All customizable shortcuts can be modified in Settings. Click the shortcut input and press the new key combination.

**Note:** Shortcuts with underlined names indicate they have been customized from the default values.

Non-editable shortcuts (system reserved):
- Esc - Close current dialog/window
- F1 - Toggle help
- Arrow keys - Navigate in lists

### AI Configuration

Configure AI models for natural language assistance:

- **Enable AI** - Turn AI features on/off
- **Active Model** - Select which AI model to use
- **Confirm before execute** - Show confirmation dialog before executing AI-generated commands
- **Max turns** - Maximum conversation rounds in Agent mode
- **Render markdown** - Format AI responses with markdown

**Adding a Model:**
1. Click "+ Add Model"
2. Enter model name (for display)
3. Select provider (OpenAI, Azure, etc.)
4. Enter model ID (e.g., `gpt-4`, `gpt-3.5-turbo`)
5. Enter API URL (or use default)
6. Enter API Key
7. Adjust temperature (0.0-2.0, lower = more deterministic)
8. Click "Test Connection" to verify
9. Click "Save"

---

## Online Docs

For more details and advanced usage, visit our online documentation.