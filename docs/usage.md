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

## Keyboard Shortcuts

Rudist supports customizable keyboard shortcuts. Default configuration:

| Shortcut | Function | Description |
|----------|----------|-------------|
| `Ctrl/Cmd + T` | New Tab | Open a new blank tab |
| `Ctrl/Cmd + W` | Close Tab | Close the current active tab |
| `Ctrl/Cmd + R` | Refresh Key | Reload the currently selected key |
| `Ctrl/Cmd + F` | Focus Filter | Quickly focus the key filter input box |
| `Ctrl/Cmd + ,` | Open Settings | Open the settings window |
| `Esc` | Close Settings | Close the settings window (only when settings is open) |

> **Note**: Use `Cmd` key on macOS, `Ctrl` key on Windows/Linux.

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
