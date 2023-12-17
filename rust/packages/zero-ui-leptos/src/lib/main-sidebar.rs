use leptos::*;
use leptos_icons::*;
use zero_ui::prelude::{Icon, Sidebar};

#[component]
pub fn MainSidebar() -> impl IntoView {
    view! {
        <Sidebar
            top_icons=|| {
                view! {}
            }

            bottom_icons=|| {
                view! {
                    <Icon icon=FiIcon::FiSearch/>
                    <Icon icon=FiIcon::FiUser/>
                    <Icon icon=FiIcon::FiMessageCircle/>
                    <Icon icon=FiIcon::FiSettings/>
                }
            }
        />
    }
}

