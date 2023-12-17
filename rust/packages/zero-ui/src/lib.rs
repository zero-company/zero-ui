#[path = "unpack-assets.rs"]
pub mod unpack_assets;
pub use unpack_assets::unpack_assets;

#[path = "lib/app-container.rs"]
pub mod app_container;
#[path = "lib/app-layout.rs"]
pub mod app_layout;
#[path = "lib/icon.rs"]
pub mod icon;
#[path = "lib/sidebar.rs"]
pub mod sidebar;
#[path = "lib/text.rs"]
pub mod text;
#[path = "lib/zero-icon.rs"]
pub mod zero_icon;

pub mod prelude {
    pub use super::app_container::*;
    pub use super::app_layout::*;
    pub use super::icon::*;
    pub use super::sidebar::*;
    pub use super::text::*;
    pub use super::zero_icon::*;
}

pub fn is_normal_types<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn normal_types() {
    is_normal_types::<text::TextVariant>();
}

