use leptos::*;

/*
Tailwind Include:
zero-divide-x
zero-divide-y
*/

/// SuperAppLayout Leptos Component
#[component]
pub fn SuperAppLayout(children: Children) -> impl IntoView {
    view! {
        <div
            id="SuperAppLayout"
            class="flex h-screen w-screen overflow-clip font-notoSans text-xs bg-zinc-900 text-zinc-400"
        >
            {children()}
        </div>
    }
}

