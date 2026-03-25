# Usage Guide

## Interface Overview

Rudist uses a multi-tab design with four main areas:

- **Top Panel**: Connection management, database selection, settings access
- **Left Sidebar**: Key list, filter search
- **Central Panel**: Key value viewing and editing
- **Bottom Status Bar**: Connection status, key count statistics

## Connection Management

### Creating a New Connection

1. Click the **"+ New"** button in the top panel
2. Fill in the connection details in the dialog:
   - **Connection Name**: Custom identifier (e.g., "Production", "Local Dev")
   - **Host Address**: Redis server IP or hostname
   - **Port**: Default is 6379
   - **Username** (optional): ACL authentication username
   - **Password** (optional): Authentication password
   - **Color Marker**: Choose a high-contrast color to distinguish environments
3. Click **Save** to save the connection configuration

### Connect/Disconnect

- Select a saved connection from the dropdown menu to automatically connect
- Click the **Disconnect** button next to the connection name to disconnect
- Supports multiple tabs connecting to different Redis instances simultaneously

### Edit/Delete Connections

- Hover over a connection item in the dropdown and click the edit icon to modify
- Click the delete icon to remove a saved connection

## Database Operations

### Switching Databases

After connecting, use the **DB** dropdown in the top panel to switch databases (0-15).

### Browsing Keys

The **left sidebar** displays all keys in the current database:

- **Load Statistics**: Shows loaded keys / total keys count
- **Scroll Loading**: Loads keys in batches, automatically loads more when scrolling
- **Load All**: Click the load button to load all keys at once (use with caution for large datasets)

### Key Filtering

Enter keywords in the filter box at the top of the left sidebar:

- Supports `*` wildcard (e.g., `user:*` matches all user-related keys)
- Automatically adds wildcards to both ends if no `*` is present
- Clear the filter to display all keys

## Key Operations

### Viewing Key Values

1. Click a key name in the left sidebar
2. The central panel displays detailed key information:
   - **Key Name** (copyable)
   - **Data Type** (String, List, Hash, Set, ZSet)
   - **TTL** expiration time
   - **Value Content**

### Data Type Support

| Type | Display Format | Special Features |
|------|----------------|------------------|
| **String** | Text editor | View full content directly |
| **List** | Indexed list | Lazy loading, click elements to edit |
| **Hash** | Field-value table | Field filter search, field-level editing |
| **Set** | Member list | Lazy loading, click elements to edit |
| **ZSet** | Score-member list | Display sorted by score |

### Editing Keys

1. Click the **Edit** button to enter edit mode
2. Modify the key name or value content
3. Click **Save** to apply changes, or **Cancel** to discard

### Creating New Keys

1. Click the **➕** button at the top of the left sidebar
2. Select the data type (String, List, Hash, Set, ZSet)
3. Enter the key name, initial value, and TTL (-1 for no expiration)
4. Click **Create** to create the key

### Deleting Keys

- Click the **Delete** button in view mode to delete the current key
- Deletion is irreversible, please proceed with caution

### Refreshing Keys

Click the **🔄** button to reload the current key's value and TTL.

### Modifying TTL

1. Click the edit button next to the TTL
2. Enter the new expiration time in seconds
3. Click **Save** to apply, or leave empty to remove expiration

## AI Assistant

Rudist integrates an AI assistant that helps you manage Redis databases using natural language. Instead of memorizing Redis commands, you can describe what you want in plain English.

### Opening the AI Panel

Press `Ctrl/Cmd + E` to open the command line panel at the bottom of the window. The panel includes:

- **History area**: Shows previous commands and AI responses
- **Input field**: Enter Redis commands or natural language queries

### How It Works

The AI panel intelligently detects your input type:

| Input Type | Example | Behavior |
|------------|---------|----------|
| **Redis Command** | `GET mykey` | Directly executes the command |
| **Natural Language** | `show me all string keys` | Sends to AI, returns a Redis command for confirmation |

**Smart Detection**: If your input matches a known Redis command (like `GET`, `SET`, `DEL`), it's executed directly. Otherwise, it's sent to the AI for interpretation.

### AI Command Confirmation

When you enter a natural language query and the AI returns a Redis command:

1. A confirmation dialog appears showing:
   - **Q**: Your original question
   - **→**: The suggested Redis command
2. Choose to **Execute** or **Cancel**

**Keyboard shortcuts for the dialog:**
- `→` (Right Arrow): Execute the suggested command
- `←` (Left Arrow): Cancel and discard

**Auto-execute mode**: In AI settings, you can enable auto-execute to skip the confirmation dialog and run commands directly.

### AI Context Awareness

The AI has access to your current context when generating commands:

- Current connection name and address
- Current database number
- Currently selected key (if any)

This allows it to generate context-aware commands like `GET user:123` when you've selected that key.

### Configuring AI

1. Open Settings (`Ctrl/Cmd + ,`)
2. Go to the **AI** tab
3. Configure:
   - **Enable/Disable**: Toggle AI features globally
   - **Model Selection**: Choose from configured AI models
   - **Confirm Before Execute**: Require confirmation before running AI-generated commands
   - **Show Thinking**: Display "Thinking..." while AI is processing
   - **System Prompt**: Customize AI behavior instructions

### Adding AI Models

Rudist supports multiple AI providers via custom API endpoints:

1. Click **Add Model**
2. Fill in:
   - **Name**: Display name (e.g., "My GPT-4")
   - **URL**: API endpoint base URL
   - **Model ID**: Specific model identifier
   - **API Key**: Authentication key (stored securely in system keychain)

**Supported providers**: OpenAI, Anthropic Claude, Ollama (local), OpenRouter, and any OpenAI-compatible API.

> **Note**: API keys are stored securely in your operating system's credential store (macOS Keychain, Windows Credential Manager, Linux Secret Service). They are never saved to plain text config files.

## Keyboard Shortcuts

Rudist supports customizable keyboard shortcuts. Default configuration:

| Shortcut | Function | Description |
|----------|----------|-------------|
| `Ctrl/Cmd + T` | New Tab | Open a new blank tab |
| `Ctrl/Cmd + W` | Close Tab | Close the current active tab |
| `Ctrl/Cmd + N` | New Connection | Open the new connection dialog |
| `Ctrl/Cmd + R` | Refresh Key | Reload the currently selected key's value and TTL |
| `Ctrl/Cmd + F` | Focus Filter | Quickly focus the key filter input box |
| `Ctrl/Cmd + E` | Toggle Command Line | Open or close the CLI panel (AI command input) |
| `Ctrl/Cmd + ,` | Open Settings | Open the settings window |
| `Ctrl/Cmd + Shift + D` | Remove Duplicate Tabs | Close duplicate tabs connected to the same Redis instance |
| `F5` | Refresh Key List | Reload the entire key list from the sidebar |
| `Esc` | Close Settings / Command Line | Close settings window, or close CLI panel (context-dependent) |
| `Cmd/Ctrl + 1-9` | Switch to Tab | Switch to tab 1 through 9 |
| `Cmd/Ctrl + 0` | Switch to Last Tab | Switch to the previously active tab |
| `1-9` | Quick Connect | Connect to saved connection 1 through 9 (no modifier) |
| `0` | Connect All Unclosed | Reconnect all previously connected connections |
| `→` (Right Arrow) | Execute AI Command | Execute the AI-suggested command (non-customizable) |
| `←` (Left Arrow) | Cancel AI Command | Cancel the AI command confirmation dialog (non-customizable) |

> **Note**:
> - Use `Cmd` key on macOS, `Ctrl` key on Windows/Linux.
> - `Ctrl/Cmd + R` only refreshes the currently selected key; `F5` refreshes the entire key list in the sidebar.
> - `Esc` behavior depends on context: when settings is open, it closes settings; otherwise, it closes the CLI panel (if open).
> - Some shortcuts (Tab switching 1-9, Quick Connect 0-9, AI commands, Refresh Key List) are fixed and cannot be customized.

### Customizing Shortcuts

1. Open the settings window (`Ctrl/Cmd + ,`)
2. Go to the **Shortcuts** tab
3. Click the shortcut entry you want to modify
4. Press the new keyboard combination
5. The system automatically detects conflicts and warns
6. Click **Reset to Default** to restore default settings

## Settings

### Language Settings

- Supports **English** and **中文 (Chinese)**
- Switch in the **General** tab of the settings window
- Changes take effect immediately

### Interface Settings

- **Window Size and Position**: Automatically saves state from last session
- **Maximized State**: Remembers if window was maximized

### Shortcut Settings

- View and modify all shortcut bindings
- Supports letters, numbers, F1-F12 function keys
- Supports modifier combinations (Ctrl/Cmd, Alt, Shift)

## Status Bar Information

The bottom status bar displays the following information:

| Item | Description |
|------|-------------|
| **Connection Status** | Disconnected / Connecting / Ready / Loading |
| **Total Keys** | Total key count in current database (shown for full scan) |
| **Loaded Keys** | Number of keys loaded into the list (red indicates more available) |
| **Error Message** | Displays operation error messages (max 2 lines) |

## Multi-Tab Operations

- Each tab is an independent Redis connection session
- Click tabs to switch between different connections
- Tabs display connection names and custom color markers
- A new blank tab is automatically created after closing the last tab

## Best Practices

1. **Environment Marking**: Set prominent red markers for production environment connections
2. **Key Naming Convention**: Use consistent prefixes (e.g., `app:module:key`) for easier filtering
3. **Large Dataset Handling**: Avoid loading millions of keys at once, use filtering to narrow scope
4. **Regular Refresh**: Use `Ctrl/Cmd + R` to quickly refresh keys that may change
