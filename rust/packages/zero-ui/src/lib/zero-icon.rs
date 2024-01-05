use leptos::*;

pub const ZERO_ICON_SVG_CHILD: &str = r###"<path
fill-rule="evenodd"
clip-rule="evenodd"
d="M160 320C248.366 320 320 248.366 320 160C320 71.6344 248.366 0 160 0C71.6344 0 0 71.6344 0 160C0 248.366 71.6344 320 160 320ZM160 272C221.856 272 272 221.856 272 160C272 98.1441 221.856 48 160 48C98.1441 48 48 98.1441 48 160C48 221.856 98.1441 272 160 272Z"
fill="currentColor"
></path>"###;

#[component]
pub fn ZeroIcon(
    #[prop(into, optional)] svg_viewbox: Option<MaybeSignal<String>>,
    #[prop(into, optional)] size: Option<MaybeSignal<String>>,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.clone().unwrap_or("20".into())
            height=size.clone().unwrap_or("20".into())
            viewBox=svg_viewbox.clone().unwrap_or("0 0 24 24".into())
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            inner_html=ZERO_ICON_SVG_CHILD
        ></svg>
    }
}

