use leptos::*;

pub enum TextVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Span,
}

#[component]
pub fn Text(
    #[prop(into, optional)] variant: Option<TextVariant>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    match variant {
        None => Span(SpanProps {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        Some(validVariant) => match validVariant {
            TextVariant::H1 => view! {
                <h1 id=id class=class style=style>
                    {children()}
                </h1>
            }
            .into_view(),
            TextVariant::H2 => view! {
                <h2 id=id class=class style=style>
                    {children()}
                </h2>
            }
            .into_view(),
            TextVariant::H3 => view! {
                <h3 id=id class=class style=style>
                    {children()}
                </h3>
            }
            .into_view(),
            TextVariant::H4 => view! {
                <h4 id=id class=class style=style>
                    {children()}
                </h4>
            }
            .into_view(),
            TextVariant::H5 => view! {
                <h5 id=id class=class style=style>
                    {children()}
                </h5>
            }
            .into_view(),
            TextVariant::H6 => view! {
                <h6 id=id class=class style=style>
                    {children()}
                </h6>
            }
            .into_view(),
            TextVariant::P => view! {
                <p id=id class=class style=style>
                    {children()}
                </p>
            }
            .into_view(),
            _ => Span(SpanProps {
                id,
                class,
                style,
                children,
            })
            .into_view(),
        },
    }
}

#[component]
pub fn Span(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <span id=id class=class style=style>
            {children()}
        </span>
    }
}

