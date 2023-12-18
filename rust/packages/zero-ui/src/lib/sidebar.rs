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
    view! {
        <div id="Sidebar" class="flex flex-col h-screen" style=style>
            {top_icons()}
            <div class="grow"></div>
            {bottom_icons()}
        </div>
    }
}

