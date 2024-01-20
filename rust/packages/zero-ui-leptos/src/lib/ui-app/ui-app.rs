// use crate::prelude::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use zero_ui::prelude::{Icon, Sidebar, SubAppLayout, ZERO_ICON_SVG_CHILD};

#[path = "ui-index-page.rs"]
pub mod ui_index_page;
pub use ui_index_page::*;

#[component]
fn UiSidebar() -> impl IntoView {
    view! {
        <Sidebar
            menu=|| {
                view! {
                    <div class="flex flex-1 flex-col zero-divide-y">
                        <div class="p-2">
                            <p>"zero-ui/ui"</p>
                        </div>
                        <div class="p-2">
                            <A href="/">
                                <strong>"index"</strong>
                            </A>
                        </div>
                        <div class="p-2">
                            <A href="/button">
                                <strong>"button"</strong>
                            </A>
                        </div>
                        <div class="p-2">
                            <A href="/blog">
                                <strong>"blog"</strong>
                            </A>
                        </div>
                        <div class="p-2">
                            <A href="/ui">
                                <strong>"ui"</strong>
                            </A>
                        </div>
                        <div class="p-2"></div>

                    </div>
                }
            }

            top_icons=|| {
                view! { <Icon size="20" svg_viewbox="0 0 320 320" svg_child=ZERO_ICON_SVG_CHILD/> }
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

#[component(transparent)]
pub fn UiApp() -> impl IntoView {
    view! {
        <Route
            path="/ui"
            view=|| {
                view! {
                    <SubAppLayout
                        sidebar=|| view! { <UiSidebar/> }
                        body=|| {
                            view! { <Outlet/> }
                        }
                    />
                }
            }
        >

            <Route path="/" view=|| view! { <UiIndexPage/> }/>
        </Route>
    }
}

