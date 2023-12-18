use leptos::*;
use leptos_icons::*;
use zero_ui::prelude::{Icon, Sidebar};

#[component]
pub fn MainSidebar() -> impl IntoView {
    view! {
        <Sidebar
            top_icons=|| {
                view! {
                    <div inner_html=include_str!(
                        "./../../zero-ui-assets/icons/zero-logo-v1.svg",
                    )></div>
                }
            }

            bottom_icons=|| {
                view! {
                    <Icon size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiSearch) size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiUser) size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiMessageCircle) size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiSettings) size="20"/>
                }
            }
        />
    }
}

