# Keyboard Shortcuts

Rudist supports customizable keyboard shortcuts. Default configuration:

## Default Shortcuts

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

## Notes

- Use `Cmd` key on macOS, `Ctrl` key on Windows/Linux
- `Ctrl/Cmd + R` only refreshes the currently selected key; `F5` refreshes the entire key list
- `Esc` behavior depends on context: when settings is open, it closes settings; otherwise, it closes the CLI panel (if open)
- Some shortcuts (Tab switching 1-9, Quick Connect 0-9, AI commands, Refresh Key List) are fixed and cannot be customized

## Customizing Shortcuts

1. Open the settings window (`Ctrl/Cmd + ,`)
2. Go to the **Shortcuts** tab
3. Click the shortcut entry you want to modify
4. Press the new keyboard combination
5. The system automatically detects conflicts and warns
6. Click **Reset to Default** to restore default settings

### Shortcut Customization Indicator

- Shortcuts with **underlined names** indicate they have been customized from their default values
- This helps you quickly identify which shortcuts you've modified
- To see the original default value, hover over the shortcut entry