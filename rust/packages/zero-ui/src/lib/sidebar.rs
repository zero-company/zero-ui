use leptos::*;

#[component]
pub fn Sidebar<T, TIV, B, BIV>(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    top_icons: T,
    bottom_icons: B,
) -> impl IntoView
where
    T: Fn() -> TIV,
    TIV: IntoView,
    B: Fn() -> BIV,
    BIV: IntoView,
{
    /*
    let parsed_top_icons = match top_icons {
        Some(icons) => icons,
        None => "".into(),
    };
     */

    view! {
        <div id=id class=class style=style>
            {top_icons()}
            {bottom_icons()}
        </div>
    }
}

