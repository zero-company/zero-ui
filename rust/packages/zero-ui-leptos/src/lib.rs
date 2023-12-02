pub mod app;
#[path = "lib/app-container.rs"]
pub mod app_container;

pub mod prelude {
    pub use super::app_container::*;
}

