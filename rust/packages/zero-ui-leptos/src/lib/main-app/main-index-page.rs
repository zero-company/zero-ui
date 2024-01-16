use leptos::*;
use zero_ui::prelude::Tabs;

#[component]
pub fn MainIndexPage() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("Tab1".to_string());

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Main Index Page"</h2>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| {
                    set_active_tab.update(|active_tab| *active_tab = "Tab2".to_string())
                }
            >

                "active_tab: "
                {move || { active_tab() }}
                "--"
            </button>

            <Tabs tabs=[
                view! { <p class="p-6 text-4xl">"Tab1"</p> },
                view! { <p class="p-6 text-4xl">"Tab2"</p> },
            ]/>

        // activeTab=active_tab
        </div>
    }
}

