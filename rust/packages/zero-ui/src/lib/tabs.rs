use leptos::*;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: &'static str,
    pub content: View,
}

/// Tabs Leptos Component
#[component]
pub fn Tabs<IV>(
    #[prop(into)] tabs: Vec<IV>,
    #[prop(optional)] activeTab: Option<ReadSignal<String>>,
) -> impl IntoView
where
    IV: IntoView,
{
    view! {
        <div id="Tabs" class="">
            <p class="p-6">
                "active_tab2:"
                {move || {
                    match activeTab {
                        Some(activeTab) => activeTab(),
                        None => "N/A".to_string(),
                    }
                }}

            </p>
            {tabs.into_iter().map(|tab| view! { <div>{tab}</div> }).collect_view()}
        </div>
    }
}

// TODO: Add tabs array default for const generic

