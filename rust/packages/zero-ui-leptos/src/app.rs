use crate::prelude::Sidebar;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use zero_ui::prelude::AppContainer;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <AppContainer>
            <div class="my-0 mx-auto max-w-3xl text-center">
                <h2 class="p-6 text-4xl">"Under Development"</h2>
                <Sidebar id="Sidebar">"Text2"</Sidebar>
                <button
                    class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                    on:click=move |_| set_count.update(|count| *count += 1)
                >
                    "We've had | "
                    {move || { if count() == 0 { "N/A".to_string() } else { count().to_string() } }}

                    " | visitors"
                </button>
            </div>
        </AppContainer>
    }
}

