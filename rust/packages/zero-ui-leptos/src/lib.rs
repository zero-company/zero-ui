pub mod app;
#[path = "pages/home-page.rs"]
pub mod home_page;
#[path = "lib/main-sidebar.rs"]
pub mod main_sidebar;

pub mod prelude {
    pub use super::home_page::*;
    pub use super::main_sidebar::*;
}

