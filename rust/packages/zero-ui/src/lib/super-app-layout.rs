use crate::prelude::{Icon, Text, ZERO_ICON_SVG_CHILD};
use leptos::*;

/*
Tailwind Include:
zero-divide-x
zero-divide-y
*/

#[component]
pub fn PreloadPage() -> impl IntoView {
    view! {
        <div id="PreloadPage" class="flex flex-col h-screen w-screen justify-center items-center">
            <Icon
                size="40"
                class="flex shrink-0 justify-center items-center hover:brightness-150 pb-4"
                svg_viewbox="0 0 320 320"
                svg_child=ZERO_ICON_SVG_CHILD
            />
            <Text>"Loading..."</Text>
        </div>
    }
}

/// SuperAppLayout Leptos Component
#[component]
pub fn SuperAppLayout(#[prop(optional)] children: Option<ChildrenFn>) -> impl IntoView {
    view! {
        <div
            id="SuperAppLayout"
            class="flex h-screen w-screen overflow-clip font-notoSans text-xs bg-zinc-900 text-zinc-400"
        >
            {match children {
                Some(children) => children().into_view(),
                None => PreloadPage().into_view(),
            }}

        </div>
    }
}

// TODO: Display zero-logo svg by default with "Zero", with specific process loading like sims 4, maybe add a footer

