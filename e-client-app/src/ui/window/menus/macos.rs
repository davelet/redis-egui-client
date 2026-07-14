use e_client_config::language::Language;
use e_client_config::translations::{TranslationKey, tr};
use muda::accelerator::Accelerator;
use muda::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem, Submenu};
use std::sync::OnceLock;

// Global menu state
static MENU_INITIALIZED: OnceLock<()> = OnceLock::new();

/// Menu action IDs matching the application's functionality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    // App menu (Rudist)
    CheckForUpdates,
    Preferences,
    Quit,

    // File menu
    NewConnection,
    NewTab,
    CloseTab,
    ConnectAllUnclosed,

    // Operation menu
    NewKey,
    DeleteKey,
    RefreshKeys,
    RefreshCurrentKey,
    Find,

    // View menu
    ToggleCommandLine,
    ToggleLiveLogs,
    RemoveDuplicateTabs,
    ShowAllTabs,

    // Window menu
    Minimize,
    Zoom,
    NextTab,
    PreviousTab,
    BringAllToFront,

    // Help menu
    Help,
    LocalHelp,
}

impl MenuAction {
    pub fn menu_id(&self) -> MenuId {
        let id_str = match self {
            MenuAction::CheckForUpdates => "app.check_updates",
            MenuAction::Preferences => "app.preferences",
            MenuAction::Quit => "app.quit",
            MenuAction::NewConnection => "file.new_connection",
            MenuAction::NewTab => "file.new_tab",
            MenuAction::CloseTab => "file.close_tab",
            MenuAction::ConnectAllUnclosed => "file.connect_all_unclosed",
            MenuAction::NewKey => "operation.new_key",
            MenuAction::DeleteKey => "operation.delete_key",
            MenuAction::RefreshKeys => "operation.refresh_keys",
            MenuAction::RefreshCurrentKey => "operation.refresh_current_key",
            MenuAction::Find => "operation.find",
            MenuAction::ToggleCommandLine => "view.toggle_cmd",
            MenuAction::ToggleLiveLogs => "view.toggle_logs",
            MenuAction::RemoveDuplicateTabs => "view.remove_duplicate_tabs",
            MenuAction::ShowAllTabs => "view.show_all_tabs",
            MenuAction::Minimize => "window.minimize",
            MenuAction::Zoom => "window.zoom",
            MenuAction::NextTab => "window.next_tab",
            MenuAction::PreviousTab => "window.prev_tab",
            MenuAction::BringAllToFront => "window.bring_all_to_front",
            MenuAction::Help => "help.help",
            MenuAction::LocalHelp => "help.local",
        };
        MenuId::new(id_str)
    }
}

/// Global menu instance for macOS
static mut GLOBAL_MENU: Option<Menu> = None;

/// Initialize the macOS native menu bar
pub fn initialize_menu_bar(menu_sender: std::sync::mpsc::Sender<MenuAction>, lang: Language) {
    MENU_INITIALIZED.get_or_init(|| {
        let menu = create_menu_bar(menu_sender, lang).expect("Failed to create menu bar");
        menu.init_for_nsapp();
        unsafe {
            GLOBAL_MENU = Some(menu);
        }
    });
}

/// Update the macOS native menu bar with new language
pub fn update_menu_bar_language(_lang: Language) {
    // We need to recreate the menu with new translations
    // First, we need to get the existing menu sender from somewhere?
    // For now, we'll just reinitialize with the existing sender, but we need to store it
    // TODO: Implement menu language update
}

/// Helper to create an accelerator from a string
fn accel(s: &str) -> Option<Accelerator> {
    s.parse().ok()
}

/// Create the complete macOS menu bar
fn create_menu_bar(
    menu_sender: std::sync::mpsc::Sender<MenuAction>,
    lang: Language,
) -> muda::Result<Menu> {
    let menu = Menu::new();

    // 1. App Menu (Rudist)
    let app_menu = Submenu::new("Rudist", true);
    app_menu.append_items(&[
        &MenuItem::with_id(
            MenuAction::CheckForUpdates.menu_id(),
            tr(TranslationKey::MenuCheckForUpdates, lang),
            true,
            None,
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            MenuAction::Preferences.menu_id(),
            tr(TranslationKey::MenuPreferences, lang),
            true,
            accel("Cmd+,"),
        ),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::quit(Some(tr(TranslationKey::MenuQuit, lang))),
    ])?;
    menu.append(&app_menu)?;

    // 2. File Menu
    let file_menu = Submenu::new(tr(TranslationKey::MenuFile, lang), true);
    file_menu.append_items(&[
        &MenuItem::with_id(
            MenuAction::NewConnection.menu_id(),
            tr(TranslationKey::MenuNewConnectionDots, lang),
            true,
            accel("Cmd+N"),
        ),
        &MenuItem::with_id(
            MenuAction::NewTab.menu_id(),
            tr(TranslationKey::MenuNewTab, lang),
            true,
            accel("Cmd+T"),
        ),
        &MenuItem::with_id(
            MenuAction::CloseTab.menu_id(),
            tr(TranslationKey::MenuCloseTab, lang),
            true,
            accel("Cmd+W"),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            MenuAction::ConnectAllUnclosed.menu_id(),
            tr(TranslationKey::MenuConnectAllUnclosed, lang),
            true,
            accel("Cmd+Shift+O"),
        ),
    ])?;
    menu.append(&file_menu)?;

    // 3. Operation Menu
    let operation_menu = Submenu::new(tr(TranslationKey::MenuOperation, lang), true);
    operation_menu.append_items(&[
        &MenuItem::with_id(
            MenuAction::NewKey.menu_id(),
            tr(TranslationKey::MenuNewKeyDots, lang),
            true,
            accel("Cmd+Shift+N"),
        ),
        &MenuItem::with_id(
            MenuAction::DeleteKey.menu_id(),
            tr(TranslationKey::MenuDeleteKey, lang),
            true,
            accel("Cmd+Backspace"),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            MenuAction::RefreshKeys.menu_id(),
            tr(TranslationKey::MenuRefreshKeys, lang),
            true,
            accel("Cmd+R"),
        ),
        &MenuItem::with_id(
            MenuAction::RefreshCurrentKey.menu_id(),
            tr(TranslationKey::MenuRefreshCurrentKey, lang),
            true,
            accel("Cmd+Shift+R"),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            MenuAction::Find.menu_id(),
            tr(TranslationKey::MenuFilterDots, lang),
            true,
            accel("Cmd+F"),
        ),
    ])?;
    menu.append(&operation_menu)?;

    // 4. View Menu
    let view_menu = Submenu::new(tr(TranslationKey::MenuView, lang), true);
    view_menu.append_items(&[
        &MenuItem::with_id(
            MenuAction::ToggleCommandLine.menu_id(),
            tr(TranslationKey::MenuToggleCommandLine, lang),
            true,
            accel("Cmd+L"),
        ),
        &MenuItem::with_id(
            MenuAction::ToggleLiveLogs.menu_id(),
            tr(TranslationKey::MenuToggleLiveLogs, lang),
            true,
            accel("Cmd+Shift+L"),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            MenuAction::RemoveDuplicateTabs.menu_id(),
            tr(TranslationKey::MenuRemoveDuplicateTabs, lang),
            true,
            accel("Cmd+Shift+D"),
        ),
        &MenuItem::with_id(
            MenuAction::ShowAllTabs.menu_id(),
            tr(TranslationKey::MenuShowAllTabs, lang),
            true,
            None,
        ),
    ])?;
    menu.append(&view_menu)?;

    // 5. Window Menu
    let window_menu = Submenu::new(tr(TranslationKey::MenuWindow, lang), true);
    window_menu.append_items(&[
        &PredefinedMenuItem::minimize(Some(tr(TranslationKey::MenuMinimize, lang))),
        &PredefinedMenuItem::maximize(Some(tr(TranslationKey::MenuZoom, lang))),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            MenuAction::NextTab.menu_id(),
            tr(TranslationKey::MenuNextTab, lang),
            true,
            accel("Cmd+Shift+Right"),
        ),
        &MenuItem::with_id(
            MenuAction::PreviousTab.menu_id(),
            tr(TranslationKey::MenuPreviousTab, lang),
            true,
            accel("Cmd+Shift+Left"),
        ),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::bring_all_to_front(Some(tr(
            TranslationKey::MenuBringAllToFront,
            lang,
        ))),
    ])?;
    menu.append(&window_menu)?;

    // 6. Help Menu
    let help_menu = Submenu::new(tr(TranslationKey::MenuHelp, lang), true);
    help_menu.append_items(&[
        &MenuItem::with_id(
            MenuAction::LocalHelp.menu_id(),
            tr(TranslationKey::MenuLocalHelp, lang),
            true,
            accel("F1"),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            MenuAction::Help.menu_id(),
            tr(TranslationKey::MenuOnlineHelp, lang),
            true,
            accel("Cmd+Shift+H"),
        ),
    ])?;
    menu.append(&help_menu)?;

    // Start menu event listener
    std::thread::spawn(move || {
        while let Ok(event) = MenuEvent::receiver().recv() {
            if let Some(action) = menu_id_to_action(&event.id) {
                let _ = menu_sender.send(action);
            }
        }
    });

    Ok(menu)
}

/// Convert a MenuId to a MenuAction
fn menu_id_to_action(id: &MenuId) -> Option<MenuAction> {
    match id.0.as_str() {
        "app.check_updates" => Some(MenuAction::CheckForUpdates),
        "app.preferences" => Some(MenuAction::Preferences),
        "app.quit" => Some(MenuAction::Quit),
        "file.new_connection" => Some(MenuAction::NewConnection),
        "file.new_tab" => Some(MenuAction::NewTab),
        "file.close_tab" => Some(MenuAction::CloseTab),
        "file.connect_all_unclosed" => Some(MenuAction::ConnectAllUnclosed),
        "operation.new_key" => Some(MenuAction::NewKey),
        "operation.delete_key" => Some(MenuAction::DeleteKey),
        "operation.refresh_keys" => Some(MenuAction::RefreshKeys),
        "operation.refresh_current_key" => Some(MenuAction::RefreshCurrentKey),
        "operation.find" => Some(MenuAction::Find),
        "view.toggle_cmd" => Some(MenuAction::ToggleCommandLine),
        "view.toggle_logs" => Some(MenuAction::ToggleLiveLogs),
        "view.remove_duplicate_tabs" => Some(MenuAction::RemoveDuplicateTabs),
        "view.show_all_tabs" => Some(MenuAction::ShowAllTabs),
        "window.minimize" => Some(MenuAction::Minimize),
        "window.zoom" => Some(MenuAction::Zoom),
        "window.next_tab" => Some(MenuAction::NextTab),
        "window.prev_tab" => Some(MenuAction::PreviousTab),
        "window.bring_all_to_front" => Some(MenuAction::BringAllToFront),
        "help.help" => Some(MenuAction::Help),
        "help.local" => Some(MenuAction::LocalHelp),
        _ => None,
    }
}
