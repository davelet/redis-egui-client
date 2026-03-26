# Changelog

## [0.4.1] - 2026-03-26

### Added
- Windows application icon support via winres crate
- macOS .app bundle distribution support in dist-workspace.toml
- Windows icon resource file generation via build.rs

### Changed
- Added apple-app installer target for macOS distribution

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
