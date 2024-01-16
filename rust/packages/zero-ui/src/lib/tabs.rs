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
        <div id="Tabs" class="">
            {tabs
                .into_iter()
                .map(|tab| {
                    view! {
                        <div class:hidden=move || {
                            active_tab.map(|active_tab| active_tab.get()).unwrap_or_default()
                                != tab.id
                        }>{tab.content}</div>
                    }
                })
                .collect_view()}

        </div>
    }
}

// TODO: Add tabs array default for const generic

