use leptos::*;
use leptos_router::*;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Index"</h2>
            <A href="/button">
                <strong>"button"</strong>
            </A>
            <A href="/">
                <strong>"index"</strong>
            </A>
        </div>
    }
}

