use leptos::*;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: String,
    pub content: View,
}

/// Tabs Leptos Component
#[component]
pub fn Tabs<IV, TIV>(
    #[prop()] tabs: Vec<(String, TIV)>,
    #[prop(into)] tabs2: Vec<IV>,
    #[prop(optional)] active_tab: Option<ReadSignal<String>>,
) -> impl IntoView
where
    IV: IntoView,
    TIV: IntoView,
{
    view! {
        <div id="Tabs" class="">
            <p>
                "active_tab2:"
                {move || {
                    match active_tab {
                        Some(active_tab) => active_tab(),
                        None => "N/A".to_string(),
                    }
                }}

            </p>
            <p>
                {move || {
                    active_tab.map(|active_tab| active_tab.get()).unwrap_or_default()
                        == "Tab1".to_string()
                }}

            </p>
            {tabs2
                .into_iter()
                .map(|tab| {
                    view! {
                        <div class:hidden=move || {
                            active_tab.map(|active_tab| active_tab.get()).unwrap_or_default()
                                != "Tab1".to_string()
                        }>

                            {tab}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

// TODO: Add tabs array default for const generic

/*
{tabs2
                .into_iter()
                .map(|tab| {
                    view! {
                        <div class:hidden=move || {
                            active_tab.map(|active_tab| active_tab.get()).unwrap_or_default()
                                != "Tab1".to_string()
                        }>

                            {tab}
                        </div>
                    }
                })
                .collect_view()}
*/

