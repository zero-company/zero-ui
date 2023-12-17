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
                    <Icon icon=FiIcon::FiSearch size="20"/>
                    <Icon icon=FiIcon::FiUser size="20"/>
                    <Icon icon=FiIcon::FiMessageCircle size="20"/>
                    <Icon icon=FiIcon::FiSettings size="20"/>
                }
            }
        />
    }
}

