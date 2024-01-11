use leptos::*;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: &'static str,
    pub content: View,
}

/// Tabs Leptos Component
#[component]
pub fn Tabs<const TLENGTH: usize>(#[prop(optional)] tabs: Option<[Tab; TLENGTH]>) -> impl IntoView {
    view! {
        <div id="Tabs" class="">
            <p>"Render Tabs"</p>
        </div>
    }
}

// TODO: Add default for const generic