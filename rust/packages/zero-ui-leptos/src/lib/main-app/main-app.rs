// use crate::prelude::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use zero_ui::prelude::{Icon, Sidebar, SubAppLayout, Tab, Tabs, ZERO_ICON_SVG_CHILD};

#[path = "main-index-page.rs"]
pub mod main_index_page;
pub use main_index_page::*;

#[component]
fn MainSidebar() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("MainTab".to_string());

    view! {
        <Sidebar
            menu=|| {
                view! {
                    <Tabs
                        tabs=vec![
                            Tab {
                                id: "MainTab".to_string(),
                                content: view! { <p class="p-6 text-4xl">"MainTab"</p> }.into_view(),
                            },
                            Tab {
                                id: "SearchTab".to_string(),
                                content: view! { <p class="p-6 text-4xl">"SearchTab"</p> }
                                    .into_view(),
                            },
                            Tab {
                                id: "Tab3".to_string(),
                                content: view! { <p class="p-6 text-4xl">"Tab3"</p> }.into_view(),
                            },
                        ]

                        active_tab=active_tab
                    />
                }
            }

            top_icons=|| {
                view! {
                    <Icon
                        size="20"
                        svg_viewbox="0 0 320 320"
                        svg_child=ZERO_ICON_SVG_CHILD
                        on_click=move |_| {
                            set_active_tab.update(|active_tab| *active_tab = "MainTab".to_string())
                        }
                    />
                }
            }

            bottom_icons=|| {
                view! {
                    <Icon
                        leptos_icons_icon=Icon::from(FiIcon::FiSearch)
                        size="20"
                        on_click=move |_| {
                            set_active_tab
                                .update(|active_tab| *active_tab = "SearchTab".to_string())
                        }
                    />

                    <Icon
                        leptos_icons_icon=Icon::from(FiIcon::FiUser)
                        size="20"
                        on_click=move |_| {
                            set_active_tab.update(|active_tab| *active_tab = "UserTab".to_string())
                        }
                    />

                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiMessageCircle) size="20"/>
                    <Icon leptos_icons_icon=Icon::from(FiIcon::FiSettings) size="20"/>
                }
            }
        />
    }
}

#[component(transparent)]
pub fn MainApp() -> impl IntoView {
    view! {
        <Route
            path="/"
            view=|| {
                view! {
                    <SubAppLayout
                        sidebar=|| view! { <MainSidebar/> }
                        body=|| {
                            view! { <Outlet/> }
                        }
                    />
                }
            }
        >

            <Route path="/" view=|| view! { <MainIndexPage/> }/>
            <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
        </Route>
    }
}

