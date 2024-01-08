use crate::prelude::*;
use leptos::*;
use leptos_router::*;
use zero_ui::prelude::SubAppLayout;

#[component]
pub fn BlogAppLayout() -> impl IntoView {
    view! {
        <div>
            <p>"test1"</p>
            <Outlet/>
        </div>
    }
}

#[component(transparent)]
pub fn BlogAppRoutes() -> impl IntoView {
    view! { <Route path="/" view=|| view! { <BlogIndexPage/> }/> }
}

#[component(transparent)]
pub fn BlogApp() -> impl IntoView {
    view! {
        <Route
            path="/blog"
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
            <Route path="/" view=|| view! { <BlogIndexPage/> }/>
        </Route>
    }
}

