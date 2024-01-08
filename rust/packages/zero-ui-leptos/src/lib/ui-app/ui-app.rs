use crate::prelude::*;
use leptos::*;
use leptos_router::*;
use zero_ui::prelude::SubAppLayout;

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

