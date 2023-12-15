pub mod app;
#[path = "lib/main-sidebar.rs"]
pub mod main_sidebar;

pub mod prelude {
    pub use super::main_sidebar::*;
}

