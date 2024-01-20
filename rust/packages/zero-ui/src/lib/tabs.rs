use leptos::*;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: String,
    pub content: View,
}

/// Tabs Leptos Component
#[component]
pub fn Tabs(
    #[prop()] tabs: Vec<Tab>,
    #[prop(optional)] active_tab: Option<ReadSignal<String>>,
) -> impl IntoView {
    view! {
        <div id="Tabs" class="flex">
            {tabs
                .into_iter()
                .map(|Tab { id, content }| {
                    view! {
                        <div
                            id=&id
                            class:hidden=move || {
                                active_tab.map(|active_tab| active_tab.get()).unwrap_or_default()
                                    != *(&id)
                            }
                        >

                            {content}
                        </div>
                    }
                })
                .collect_view()}

        </div>
    }
}

// TODO: Add tabs array default for const generic

