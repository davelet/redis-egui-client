// Panels module - all UI panel rendering functions

pub mod central_panel;
pub mod side_panel;
pub mod status_bar;
pub mod tab_bar;
pub mod top_panel;

// Re-export all panel functions
pub use central_panel::render_central_panel;
pub use central_panel::render_error_panel;
pub use side_panel::render_side_panel;
pub use status_bar::render_status_bar;
pub use tab_bar::render_tab_bar;
pub use top_panel::render_top_panel;
