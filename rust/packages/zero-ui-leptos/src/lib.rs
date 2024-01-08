pub mod app;
#[path = "lib/blog-app/blog-app.rs"]
pub mod blog_app;
#[path = "lib/blog-app/blog-index-page.rs"]
pub mod blog_index_page;
#[path = "lib/main-app/main-app.rs"]
pub mod main_app;
#[path = "lib/main-app/main-index-page.rs"]
pub mod main_index_page;
#[path = "lib/main-sidebar.rs"]
pub mod main_sidebar;
#[path = "lib/ui-app/ui-app.rs"]
pub mod ui_app;
#[path = "lib/ui-app/ui-index-page.rs"]
pub mod ui_index_page;

pub mod prelude {
    pub use super::blog_app::*;
    pub use super::blog_index_page::*;
    pub use super::main_app::*;
    pub use super::main_index_page::*;
    pub use super::main_sidebar::*;
    pub use super::ui_app::*;
    pub use super::ui_index_page::*;
}

