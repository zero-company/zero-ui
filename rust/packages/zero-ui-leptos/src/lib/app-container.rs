use leptos::*;

#[component]
pub fn AppContainer(children: Children) -> impl IntoView {
    view! {
        <div
            id="AppContainer"
            class="flex h-screen w-screen overflow-clip font-notoSans bg-zinc-900 text-zinc-400 text-xs"
        >
            {children()}
        </div>
    }
}

