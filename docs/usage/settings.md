# Settings

## Theme Settings

- Support themes as System/Light/Dark
- Apply theme immediately on selection

## Language Settings

- Supports **English** and **中文 (Chinese)**
- Switch in the **General** tab of the settings window
- Changes take effect immediately

## Interface Settings

- **Window Size and Position**: Automatically saves state from last session
- **Maximized State**: Remembers if window was maximized

## Shortcut Settings

- View and modify all shortcut bindings
- Supports letters, numbers, F1-F12 function keys
- Supports modifier combinations (Ctrl/Cmd, Alt, Shift)

## Update Settings

The app automatically checks for new versions from GitHub Releases.

- **Check on startup**: Automatically check for updates when the app starts
- **Check interval**: Daily / Weekly / Monthly
- **Manual check**: Click "Check for Updates" to check immediately

When a new version is found:

- A **green version number** appears next to the settings button in the top toolbar. Click it to open the Release page.
- The Update section in Settings shows version details and actions.

**Skip version**: Click "Skip this version" to stop receiving notifications for that version. View and cancel skipped versions in Settings.

## Best Practices

1. **Environment Marking**: Set prominent red markers for production environment connections
2. **Key Naming Convention**: Use consistent prefixes (e.g., `app:module:key`) for easier filtering
3. **Large Dataset Handling**: Avoid loading millions of keys at once, use filtering to narrow scope
4. **Regular Refresh**: Use `Ctrl/Cmd + R` to quickly refresh keys that may change