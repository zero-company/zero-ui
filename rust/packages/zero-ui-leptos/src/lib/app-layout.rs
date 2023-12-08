use leptos::*;

#[component]
pub fn AppLayout<F, IV>(sidebar: F /*page: F*/) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div id="AppLayout" class="flex ">
            <div id="SidebarContainer">{sidebar()}</div>
        // <div id="ContentContainer">{page()}</div>
        </div>
    }
}

