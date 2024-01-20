use leptos::*;

/// Icon Leptos Component
///
/// Icons index: https://carlosted.github.io/icondata/
///
/// Documentation: https://github.com/Carlosted/leptos-icons
#[component]
pub fn Icon(
    #[prop(into, optional)] leptos_icons_icon: Option<MaybeSignal<leptos_icons::Icon>>,
    #[prop(into, optional)] svg_child: Option<MaybeSignal<&'static str>>,
    #[prop(into, optional)] svg_viewbox: Option<MaybeSignal<String>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_label: Option<AttributeValue>,
    #[prop(into, optional)] size: Option<MaybeSignal<String>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let svg = match (leptos_icons_icon, svg_child) {
        (Some(i), None) => leptos_icons::Icon(leptos_icons::IconProps {
            icon: i.get().into(),
            width: size.clone(),
            height: size.clone(),
            class: None,
            style: None,
        })
        .into_view(),
        // If both svg_child and leptos_icons_icon are provided, prioritize svg_child
        (Some(_), Some(s)) | (None, Some(s)) => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width=size.clone().unwrap()
                height=size.clone().unwrap()
                viewBox=svg_viewbox.clone().unwrap_or("0 0 24 24".into())
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                inner_html=s.get()
            ></svg>
        }
        .into_view(),
        (None, None) => view! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width=size.clone().unwrap()
                height=size.clone().unwrap()
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <circle cx="12" cy="12" r="10"></circle>
                <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
                <line x1="12" y1="17" x2="12.01" y2="17"></line>
            </svg>
        }
        .into_view(),
    };

    let default_class = "h-9 w-9 flex shrink-0 justify-center items-center hover:brightness-150";

    let parsed_class = match class {
        Some(signal) => signal.get(),
        None => default_class.into(),
    };

    let default_cb = Callback::new(move |_: ev::MouseEvent| {});

    view! {
        <button
            on:click=on_click.unwrap_or(default_cb)
            id=id
            class=parsed_class
            style=style
            aria_label=aria_label
        >
            {svg}
        </button>
    }
}

