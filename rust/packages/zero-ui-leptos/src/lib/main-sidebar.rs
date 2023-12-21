use leptos::*;
use leptos_icons::*;
use zero_ui::prelude::{Icon, Sidebar};

#[component]
pub fn MainSidebar() -> impl IntoView {
    let sidebar_tabs = [(1, 1), (2, 2), (3, 3)];

    view! {
        <Sidebar
            top_icons=|| {
                view! {
                    <Icon
                        size="20"
                        svg_viewbox="0 0 320 320"
                        svg_child=r###"<path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M160 320C248.366 320 320 248.366 320 160C320 71.6344 248.366 0 160 0C71.6344 0 0 71.6344 0 160C0 248.366 71.6344 320 160 320ZM160 272C221.856 272 272 221.856 272 160C272 98.1441 221.856 48 160 48C98.1441 48 48 98.1441 48 160C48 221.856 98.1441 272 160 272Z"
                        fill="currentColor"
                        ></path>"###
                    />
                }
            }

            bottom_icons=|| {
                view! {
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiSearch) size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiUser) size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiMessageCircle) size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiSettings) size="20"/>
                }
            }
        />
    }
}

