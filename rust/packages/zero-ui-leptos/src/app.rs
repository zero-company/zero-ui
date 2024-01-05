use crate::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use zero_ui::prelude::{SubAppLayout, SuperAppLayout};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <SuperAppLayout>
                <SubAppLayout
                    sidebar=|| view! { <MainSidebar/> }
                    body=|| {
                        view! {
                            <Routes>
                                <Route path="/" view=move || view! { <IndexPage/> }/>
                                <Route path="/button" view=move || view! { <ButtonPage/> }/>
                                <Route path="/*any" view=move || view! { <h1>"Not Found"</h1> }/>
                            </Routes>
                        }
                    }
                />
            </SuperAppLayout>
        </Router>
    }
}

