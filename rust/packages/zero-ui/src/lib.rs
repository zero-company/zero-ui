#[path = "lib/app-container.rs"]
pub mod app_container;
#[path = "lib/text.rs"]
pub mod text;

pub mod prelude {
    pub use super::app_container::AppContainer;
    pub use super::text::*;
}

