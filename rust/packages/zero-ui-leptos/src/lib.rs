pub mod app;
#[path = "lib/blog-app/blog-app.rs"]
pub mod blog_app;
#[path = "lib/blog-app/blog-index-page.rs"]
pub mod blog_index_page;
#[path = "pages/button-page.rs"]
pub mod button_page;
#[path = "pages/index-page.rs"]
pub mod index_page;
#[path = "lib/main-sidebar.rs"]
pub mod main_sidebar;

pub mod prelude {
    pub use super::blog_app::*;
    pub use super::blog_index_page::*;
    pub use super::button_page::*;
    pub use super::index_page::*;
    pub use super::main_sidebar::*;
}

