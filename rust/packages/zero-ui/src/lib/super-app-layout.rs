use leptos::*;

/*
Tailwind Include:
zero-divide-x
zero-divide-y
*/

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
                None => "zero".into_view(),
            }}

        </div>
    }
}

