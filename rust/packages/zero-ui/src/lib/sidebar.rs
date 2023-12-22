use leptos::*;

pub struct SidebarTab<const TLENGTH: usize, const BLENGTH: usize> {
    pub top: [(View, View); TLENGTH],
    pub bottom: [(View, View); BLENGTH],
}

#[component]
pub fn Sidebar<T, TIV, B, BIV, const TLENGTH: usize, const BLENGTH: usize>(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] style: Option<String>,
    #[prop(into, optional)] tabs: Option<SidebarTab<TLENGTH, BLENGTH>>,
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
            class=class.unwrap_or("flex flex-row zero-divide-x w-64".to_string())
            style=style
        >
            <div id="SidebarIcons" class="flex flex-col min-h-screen overflow-y-auto">
                {top_icons()}
                <div class="grow"></div>
                {bottom_icons()}
            </div>
            <div id="SidebarMenu" class="flex flex-col min-h-screen overflow-y-auto">
                <p class="p-1">"zeru-ui"</p>
            </div>

        </div>
    }
}

