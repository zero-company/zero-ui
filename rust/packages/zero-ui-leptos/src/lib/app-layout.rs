use leptos::*;

#[component]
pub fn AppLayout<F, G, IV>(sidebar: F, page: G) -> impl IntoView
where
    F: Fn() -> IV,
    G: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div id="AppLayout" class="flex ">
            <div id="SidebarContainer">{sidebar()}</div>
            <div id="ContentContainer">{page()}</div>
        </div>
    }
}

