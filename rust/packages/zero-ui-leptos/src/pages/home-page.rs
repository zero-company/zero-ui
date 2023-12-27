use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Under Development"</h2>

            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "We've had | "
                {move || { if count() == 0 { "N/A".to_string() } else { count().to_string() } }}

                " | visitors"
            </button>
        </div>
    }
}

