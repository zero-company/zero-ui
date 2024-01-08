use crate::prelude::*;
use leptos::*;
use leptos_router::*;
use zero_ui::prelude::SubAppLayout;

#[path = "ui-index-page.rs"]
pub mod ui_index_page;
pub use ui_index_page::*;

#[component(transparent)]
pub fn UiApp() -> impl IntoView {
    view! {
        <Route
            path="/ui"
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

            <Route path="/" view=|| view! { <UiIndexPage/> }/>
        </Route>
    }
}

