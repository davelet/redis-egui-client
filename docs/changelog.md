# Changelog

## [0.3.3]

### Added
- Multi-tab support for connecting to multiple Redis instances simultaneously
- Customizable keyboard shortcuts
- Hash field filtering functionality
- Connection color markers for environment differentiation
- Element-level edit dialogs (List, Hash, Set, ZSet)
- Automatic window position and size saving
- Remember last opened connections

### Changed
- Optimized key loading performance for large datasets
- Improved status bar information display
- Refactored connection management interface

### Fixed
- Fixed copy button status display issues
- Fixed Hash column width saving issues

## [0.3.2]

### Added
- Redis ACL authentication support (username + password)
- New shortcut system with customizable bindings
- Settings window for language and shortcut configuration
- English and Chinese language support

### Changed
- Upgraded egui to 0.33.3
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

### Changed
- Refactored application architecture with modular design
- Optimized asynchronous operation handling

### Fixed
- Fixed lag issues when loading large datasets
- Fixed connection timeout handling


## Version Guidelines

- **Major**: Major feature updates or architectural changes
- **Minor**: New features or significant improvements
- **Patch**: Bug fixes and minor optimizations
