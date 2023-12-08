use leptos::*;

// TODO: Tooltip
// TODO: Hover style
// TODO: SVG icons
#[component]
pub fn Icon(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <div id="Icon" id=id class=class style=style>
            {children()}
        </div>
    }
}

