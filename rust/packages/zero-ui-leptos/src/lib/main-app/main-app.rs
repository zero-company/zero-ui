use crate::prelude::*;
use leptos::*;
use leptos_router::*;
use zero_ui::prelude::SubAppLayout;

#[path = "main-index-page.rs"]
pub mod main_index_page;
pub use main_index_page::*;

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

            <Route path="/" view=move || view! { <h1>"Loading"</h1> }/>
            <Route path="/*any" view=move || view! { <h1>"Not Found"</h1> }/>
        </Route>
    }
}

