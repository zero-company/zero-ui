use leptos::*;

pub struct SidebarTabs<const TLENGTH: usize = 0, const BLENGTH: usize = 0> {
    pub top: Option<[(View, View); TLENGTH]>,
    pub bottom: Option<[(View, View); BLENGTH]>,
}

// impl Into for SidebarTabs<T> {}

#[component]
pub fn Sidebar<T, TIV, B, BIV, M, MIV /*, const TLENGTH: usize, const BLENGTH: usize*/>(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] style: Option<String>,
    /*
    #[prop(into, optional, into, default=Some(SidebarTabs{top: None, bottom: None}))] tabs: Option<
        SidebarTabs<TLENGTH, BLENGTH>,
    >,
    */
    top_icons: T,
    bottom_icons: B,
    menu: M,
) -> impl IntoView
where
    T: Fn() -> TIV,
    TIV: IntoView,
    B: Fn() -> BIV,
    BIV: IntoView,
    M: Fn() -> MIV,
    MIV: IntoView,
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
                <p class="p-1">{menu()}</p>
            </div>

        </div>
    }
}

