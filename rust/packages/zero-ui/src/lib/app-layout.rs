use leptos::*;

#[component]
pub fn AppLayout<B, BIV, S, SIV>(body: B, sidebar: S) -> impl IntoView
where
    B: Fn() -> BIV,
    BIV: IntoView,
    S: Fn() -> SIV,
    SIV: IntoView,
{
    view! {
        <div id="AppLayout" class="flex h-screen w-screen zero-divide-x">
            <div id="SidebarContainer" class="flex h-screen">
                {sidebar()}
            </div>
            <div id="BodyContainer" class="flex flex-1">
                {body()}
            </div>
        </div>
    }
}

