use leptos::*;

/// Icon Leptos Component
///
/// Icons index: https://carlosted.github.io/icondata/
///
/// Documentation: https://github.com/Carlosted/leptos-icons
#[component]
pub fn Icon(
    #[prop(into)] leptos_icons_icon: leptos_icons::Icon,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_label: Option<AttributeValue>,
    #[prop(into, optional)] size: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let child = leptos_icons::Icon(leptos_icons::IconProps {
        icon: leptos_icons_icon.into(),
        width: size.clone(),
        height: size.clone(),
        class: None,
        style: None,
    })
    .into_view();

    let default_class = "h-9 w-9 flex justify-center items-center";

    let parsed_class = match class {
        Some(signal) => signal.get(),
        None => default_class.into(),
    };

    view! {
        <div id=id class=parsed_class style=style aria_label=aria_label>
            {child}
        </div>
    }
}

