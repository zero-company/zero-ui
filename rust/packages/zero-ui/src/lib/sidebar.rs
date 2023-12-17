use leptos::*;

#[component]
pub fn Sidebar<T, TIV, B, BIV>(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    top_icons: T,
    bottom_icons: B,
) -> impl IntoView
where
    T: Fn() -> TIV,
    TIV: IntoView,
    B: Fn() -> BIV,
    BIV: IntoView,
{
    /*
    let parsed_top_icons = match top_icons {
        Some(icons) => icons,
        None => "".into(),
    };
     */

    view! {
        <div id=id class=class style=style>
            <div class="h-9 w-9 text-xl flex justify-center items-center">
                <svg
                    width="1em"
                    height="1em"
                    viewBox="0 0 320 320"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M160 320C248.366 320 320 248.366 320 160C320 71.6344 248.366 0 160 0C71.6344 0 0 71.6344 0 160C0 248.366 71.6344 320 160 320ZM160 272C221.856 272 272 221.856 272 160C272 98.1441 221.856 48 160 48C98.1441 48 48 98.1441 48 160C48 221.856 98.1441 272 160 272Z"
                        fill="currentColor"
                    ></path>
                </svg>
            </div>

            {top_icons()}
            {bottom_icons()}
        </div>
    }
}

