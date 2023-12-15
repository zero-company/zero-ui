pub mod app;
#[path = "lib/sidebar.rs"]
pub mod sidebar;

pub mod prelude {
    pub use super::sidebar::*;
}

