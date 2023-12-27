use crate::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use zero_ui::prelude::{AppContainer, AppLayout};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <AppContainer>
                <AppLayout
                    sidebar=|| view! { <MainSidebar/> }
                    body=|| {
                        view! {
                            <Routes>
                                <Route path="/" view=move || view! { <IndexPage/> }/>
                                <Route path="/button" view=move || view! { <ButtonPage/> }/>
                            </Routes>
                        }
                    }
                />

            </AppContainer>
        </Router>
    }
}

