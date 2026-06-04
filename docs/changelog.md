# Changelog

## [0.6.1] - 2026-06-04

### New Features

**Binary-Safe Hash Fields**
- Hash field values are now read as raw bytes; non-UTF-8 content (serialized protobuf, gzipped blobs, image bytes, etc.) is shown as a hex preview placeholder `[Binary data, N bytes]...` instead of failing. UTF-8 values render and edit normally.
- Hash field listing is now robust against very large hashes:
  - **Redis 7.4+** uses `HSCAN ... NOVALUES` so the server only transfers field names.
  - **Redis < 7.4** falls back to `HKEYS` with a client-side glob match. The previous plain `HSCAN` returned alternating field/value pairs and could exceed the 500 ms response timeout on multi-MB hashes over WAN; the new path keeps the per-call payload small enough to always fit in the budget.
- A 100,000-field cap prevents a pathological key from exhausting memory.

**Configurable Key-Filter Debounce**
- New **Filter debounce** setting in **Settings → Display** (range 0–2000 ms, default 300 ms). The SCAN now fires shortly after typing stops rather than on every keystroke, so rapid typing is collapsed into a single Redis round-trip.
- Setting it to 0 restores the previous "fire on every keystroke" behavior.
- The pending window is rendered as a "Loading" state and the **Load More** button is suppressed while pending, to avoid stale clicks racing a soon-to-be-replaced key set.

**Hash Field Loading State**
- Each per-field "Load" button now shows a "Loading..." state while its HGET is in flight and is disabled to reject duplicate clicks on the same (key, field) pair.
- The entry is always removed at the end of the request, including on error, to avoid leaving the button stuck.

**Asynchronous UI Repaint**
- Background tasks can now wake the UI thread directly via an `egui::Context` repaint closure installed on every tab, so lazy-load results appear without waiting for the next input event. The `needs_repaint` flag alone was insufficient when egui was idle.
- The callback defaults to a no-op for non-UI consumers (tests, CLI tools).

**i18n**
- Localized key-loading progress text: `LoadingKeysProgress`, `LoadedKeysCount`, `LoadedKeysPartial`, `AllKeysLoaded`, `LoadedKeysClickMore`.
- Localized the new debounce setting label and hint (`KeyFilterDebounce`, `KeyFilterDebounceHint`).
- All new strings have English and Chinese variants.

## [0.5.1] - 2026-04-09

### New Features

**GIM Configuration Import**
- Added "Import from GIM" button in AI settings panel
- Automatically imports AI model configuration from `~/.config/gim/config.toml`
- Auto-detects provider type from URL
- Imports API key and stores securely in system keychain
- If model already exists, replaces it with imported configuration

## [0.5.0] - 2026-04-09

### New Features

**Shortcut Customization Indicator**
- Added visual indicator for customized shortcuts in settings
- Shortcuts with underlined names indicate they have been modified from default values
- Helps users quickly identify which shortcuts they've customized

### Changed

**Settings Panel Reorganization**
- Reorganized settings into collapsible sections for better navigation
- Grouped related settings: Display, Connection, Shortcuts, AI Configuration
- Improved settings UI with clearer section headers

## [0.4.8] - 2026-04-07

### New Features

**Quick Connect Mode**
- Added Quick Connect mode in New Connection dialog
- Supports Redis URL format: `redis://[<username>][:<password>@]<hostname>[:port][/[<db>]]`
- Supports TLS connections via `rediss://` scheme
- Auto-extracts host name as default connection name
- Toggle between Quick Connect and Advanced mode

**TLS Support for Advanced Mode**
- Added "Use TLS" checkbox in Advanced connection mode
- Enables secure TLS/SSL connections without URL syntax
- Both Quick Connect and Advanced modes now fully support TLS

**Toast Notification System**
- Custom lightweight toast implementation (no third-party dependencies)
- Supports warning, error, info, success notification types
- Toasts stack in bottom-right corner with 5-second auto-dismiss
- Connection error toasts with dedup mechanism per connection + message
- Cross-frame timing handling to detect errors that arrive after loading completes

**Open Connections in New Tab Setting**
- New setting: "Open Connections in New Tab" (default: enabled)
- When disabled, clicking Connect uses current tab instead of creating new one
- Applies to Welcome page, connection list, and keyboard shortcuts

## [0.4.7] - 2026-04-03

- add theme switching support
- add 'help' button in settings window

## [0.4.6] - 2026-04-03

- Markdown list rendering: Fixed the issue where list item bullets were not on the same line as the content 
- SoftBreak handling: Fixed the issue of missing line breaks in plain text content

## [0.4.5] - 2026-04-02

- feat: add shortcut `esc` to close ai model editor

## [0.4.4] - 2026-04-01

- fix: fix the max-turn bug of the AI agent mode
- fix: support line warping in the command line panel

## [0.4.3] - 2026-03-31

### Added
- **AI Chat Mode** - New stateless AI interaction mode for direct command translation
  - No conversation context maintained (each message is independent)
  - No Redis tool access for maximum safety
  - Translates natural language to Redis commands with confirmation dialog
  - Example: "How many keys do I have?" → `DBSIZE` (requires confirmation)
- **AI Agent Mode** - Full-featured mode with tool calling capabilities
  - Stateful conversation with context maintained across messages
  - Direct access to 18 Redis tools (filter_keys, get_key_info, execute_redis_command, etc.)
  - Provides direct answers without confirmation prompts
  - Requires LLMs with tool-calling support (GPT-4o, Claude Sonnet, etc.)
- **AI Mode Toggle** - Quick mode switching in command line panel
  - Switch between Chat and Agent modes on the fly
  - Mode state is cleared when switching to ensure consistency
- **Default AI Mode Setting** - Configure your preferred mode in AI Settings

## [0.4.2] - 2026-03-30

### Added
- **AI Agent with Tool Calling** - Integrated rig-core for enhanced AI capabilities
  - Filter keys by pattern (filter_keys tool)
  - Get detailed key information (get_key_info tool)
  - Delete keys (delete_keys tool)
  - Execute Redis commands (execute_redis_command tool)
  - Get database statistics (get_db_stats tool)
  - And other tools up to 18

### Changed
- AI chat now uses rig-core Agent for better multi-round conversation support


## [0.4.1] - 2026-03-26

### Added
- Windows application icon support via winres crate
- macOS .app bundle distribution support in dist-workspace.toml
- Windows icon resource file generation via build.rs

### Changed
- Added apple-app installer target for macOS distribution
- Upgraded reqwest from 0.12 to 0.13 to support rig-core dependencies

## [0.4.0] - 2026-03-25

### Added
- Group keys by colon - Organize keys in a tree structure based on colon separator
- Command history navigation (up/down arrow keys) for the command line panel
- RefreshKeys shortcut (F5) for manual key refresh
- Async AI chat in command line panel
- Auto Refresh TTL setting
- Auto Expand Composite Types setting
- Auto Expand Threshold setting
- AI API provider support (OpenAI, Anthropic, OpenRouter, Ollama)

### Changed
- Improved settings UI with individual grid sections
- Adjusted shortcut processing to skip when text input is focused except for ESC/function keys
- Updated documentation with keychain/keyring access info and AI assistant usage

### Fixed
- Fixed UI not refreshing after keys finished loading
- Fixed copy button status display issues

## [0.3.4]

### Added
- Tab switching shortcuts (Ctrl/Cmd + 1-9) for quick tab navigation

## [0.3.3] - 2026-03-12

### Added
- Non-editable system shortcuts (CloseSettings)

### Changed
- Upgraded egui to 0.33.3
- Optimized key loading performance for large datasets
- Improved status bar information display
- Refactored connection management interface

### Fixed
- Fixed copy button status display issues
- Fixed Hash column width saving issues

## [0.3.2]

### Added
- Redis ACL authentication support (username + password)

### Changed
- Optimized connection configuration saving mechanism
- Improved error message display

### Fixed
- Fixed connection state update issues when disconnecting
- Fixed key value loading issues in certain cases

## [0.3.1]

### Added
- TTL editing functionality
- Key renaming support
- Copy Key and Copy Value buttons

### Changed
- Optimized String type value display
- Improved edit mode interface layout

### Fixed
- Fixed type selection issues when creating new keys
- Fixed default value issues when editing connection configurations

## [0.3.0]

### Added
- Complete key editing functionality (String, List, Hash, Set, ZSet)
- Key deletion functionality
- New key creation dialog
- Database switching (DB 0-15)
- Connection configuration management (CRUD)
- Open connections persistence on exit

### Changed
- Refactored application architecture with modular design
- Optimized asynchronous operation handling

### Fixed
- Fixed lag issues when loading large datasets
- Fixed connection timeout handling

## [0.2.0]

### Added
- Copy feedback functionality
- Hash column width persistence
- Hash field filtering
- Set and ZSet type display
- Key scan and progressive loading
- Connection preferences (DB, side panel width)
- Settings window (language, auto-connect)
- TTL support with UI enhancements
- Element editing for List, Hash, Set, ZSet
- Hash type table display with borders
- Release workflow & WiX installer

### Changed
- Improved font system with cross-platform support
- Refactored panel organization into panels/ subdirectory
- Improved UI constants and error handling

### Fixed
- Side panel state sharing between tabs
- Binary key handling
- Tab name and color display on connection

## [0.1.0]

### Added
- Basic Redis connection management
- Multi-tab support
- Connection color indicators
- Basic i18n support (English, Chinese)
- Color picker for connections
- Key-value viewer for basic types

## Version Guidelines

- **Major**: Major feature updates or architectural changes
- **Minor**: New features or significant improvements
- **Patch**: Bug fixes and minor optimizations
