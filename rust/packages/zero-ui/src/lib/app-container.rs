use leptos::*;

/// AppContainer Leptos Component
#[component]
pub fn AppContainer(children: Children) -> impl IntoView {
    view! {
        <div
            id="AppContainer"
            class="flex h-screen w-screen overflow-clip font-notoSans text-xs bg-zinc-900 text-zinc-400"
        >
            {children()}
        </div>
    }
}

