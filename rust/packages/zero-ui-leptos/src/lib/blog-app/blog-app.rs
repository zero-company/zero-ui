use crate::prelude::*;
use leptos::*;
use leptos_router::*;
use zero_ui::prelude::SubAppLayout;

#[path = "blog-index-page.rs"]
pub mod blog_index_page;
pub use blog_index_page::*;

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

