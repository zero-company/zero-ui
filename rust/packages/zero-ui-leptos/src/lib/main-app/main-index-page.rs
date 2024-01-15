use leptos::*;
use zero_ui::prelude::Tabs;

#[component]
pub fn MainIndexPage() -> impl IntoView {
    let (activeTab, setActiveTab) = create_signal("Tab1".to_string());

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Main Index Page"</h2>
            <Tabs
                tabs=[
                    view! { <p class="p-6 text-4xl">"Tab1"</p> },
                    view! { <p class="p-6 text-4xl">"Tab2"</p> },
                ]

                activeTab=activeTab
            />
        </div>
    }
}

