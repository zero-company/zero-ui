use leptos::*;
use zero_ui::prelude::Tabs;

#[component]
pub fn MainIndexPage() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Main Index Page"</h2>
            <Tabs tabs=[
                view! { <p class="p-6 text-4xl">"1"</p> },
                view! { <p class="p-6 text-4xl">"2"</p> },
            ]/>
        </div>
    }
}

