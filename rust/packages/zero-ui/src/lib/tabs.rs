use leptos::*;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: &'static str,
    pub content: View,
}

/// Tabs Leptos Component
#[component]
pub fn Tabs<IV>(#[prop(into)] tabs: Vec<IV>) -> impl IntoView
where
    IV: IntoView,
{
    view! {
        <div id="Tabs" class="">
            {tabs.into_iter().map(|tab| view! { <div>{tab}</div> }).collect_view()}
        </div>
    }
}

// TODO: Add tabs array default for const generic

