use leptos::*;

#[component]
pub fn Sidebar<T, TIV, B, BIV>(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] style: Option<String>,
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
        <div
            id=id.unwrap_or("Sidebar".to_string())
            class=class.unwrap_or("flex flex-col h-screen".to_string())
            style=style
        >
            {top_icons()}
            <div class="grow"></div>
            {bottom_icons()}
        </div>
    }
}

