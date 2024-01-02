use leptos::*;

/*
Tailwind Include:
zero-divide-x
zero-divide-y
*/

#[component]
pub fn PreloadPage() -> impl IntoView {
    view! {
        <div id="PreloadPage" class="flex h-screen w-screen justify-center items-center">
            <h1>"ZERO"</h1>
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

