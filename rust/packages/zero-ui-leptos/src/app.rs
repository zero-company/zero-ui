use crate::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use zero_ui::prelude::SuperAppLayout;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <SuperAppLayout>
                <Routes>
                    <MainApp/>
                    <UiApp/>
                    <BlogApp/>
                </Routes>
            </SuperAppLayout>
        </Router>
    }
}

