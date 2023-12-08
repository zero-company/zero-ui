use leptos::*;

#[component]
pub fn Sidebar(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <div id=id class=class style=style>
            {children()}
        </div>
    }
}

