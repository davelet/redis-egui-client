# Interface Overview

Rudist uses a multi-tab design with four main areas:

- **Top Panel**: Connection management, database selection, settings access
- **Left Sidebar**: Key list, filter search
- **Central Panel**: Key value viewing and editing
- **Bottom Status Bar**: Connection status, key count statistics

## Toast Notifications

Rudist displays toast notifications for important events like connection errors. Toasts appear in the bottom-right corner of the window.

### Toast Behavior

- **Auto-dismiss**: Toasts automatically disappear after 10 seconds
- **Hover to pause**: Hover your mouse over a toast to pause the timer and keep it visible longer
- **Progress bar**: A colored progress bar at the bottom shows remaining time before auto-dismiss

### Toast Types

| Type | Icon | Description |
|------|------|-------------|
| **Info** | ℹ | General information messages |
| **Warning** | ⚠ | Warning messages (e.g., connection issues) |
| **Error** | ✖ | Error messages (e.g., connection failures) |
| **Success** | ✔ | Success messages |

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