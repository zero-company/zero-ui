use leptos::*;

/// SubAppLayout Leptos Component
#[component]
pub fn SubAppLayout(children: Children) -> impl IntoView {
    view! {
        <div id="SubAppLayout" class="">
            {children()}
        </div>
    }
}

