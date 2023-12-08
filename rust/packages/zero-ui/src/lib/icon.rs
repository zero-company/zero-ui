use leptos::*;

// TODO: Tooltip
// TODO: Hover style
/// Icon component
///
/// Icons index: https://carlosted.github.io/icondata/
///
/// Documentation: https://github.com/Carlosted/leptos-icons
#[component]
pub fn Icon(
    #[prop(into)] icon: leptos_icons::Icon,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_label: Option<AttributeValue>,
) -> impl IntoView {
    let child = leptos_icons::Icon(leptos_icons::IconProps {
        icon: icon.into(),
        width: None,
        height: None,
        class: None,
        style: None,
    })
    .into_view();
    view! {
        <div id=id class=class style=style aria_label=aria_label>
            {child}
        </div>
    }
}

