use leptos::*;

#[component]
pub fn AppLayout<H, HIV, S, SIV>(header: H, sidebar: S) -> impl IntoView
where
    H: Fn() -> HIV,
    HIV: IntoView,
    S: Fn() -> SIV,
    SIV: IntoView,
{
    view! {
        <div id="AppLayout" class="flex ">
            <div id="HeaderContainer">{header()}</div>
            <div id="SidebarContainer">{sidebar()}</div>
        </div>
    }
}

