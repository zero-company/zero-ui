use leptos::*;

#[component]
pub fn AppContainer(children: Children) -> impl IntoView {
    view! {
        <div id="AppContainer" class="flex bg-zinc-900 h-screen w-screen text-zinc-400 text-xs">
            {children()}
        </div>
    }
}

