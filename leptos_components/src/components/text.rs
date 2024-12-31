use leptos::prelude::*;

pub enum TextStyle {
    Primary
}

#[component]
pub fn H1(
    #[prop(optional)]
    style: Option<TextStyle>,
    children: Children,
) -> impl IntoView {
    let style_type;
    if style.is_none() {
        style_type = TextStyle::Primary;
    } else {
        style_type = style.unwrap();
    }
    let mut stylestring = "";
    match style_type {
        TextStyle::Primary => {
            stylestring = "font-sans text-2xl";
        }
    }
    view! {
        <h1 class={stylestring}>{children()}</h1>
    }
}

#[component]
pub fn H2(
    #[prop(optional)]
    style: Option<TextStyle>,
    children: Children,
) -> impl IntoView {
    let style_type;
    if style.is_none() {
        style_type = TextStyle::Primary;
    } else {
        style_type = style.unwrap();
    }
    let mut stylestring = "";
    match style_type {
        TextStyle::Primary => {
            stylestring = "font-sans text-xl";
        }
    }
    view! {
        <h2 class={stylestring}>{children()}</h2>
    }
}

#[component]
pub fn H3(
    #[prop(optional)]
    style: Option<TextStyle>,
    children: Children,
) -> impl IntoView {
    let style_type;
    if style.is_none() {
        style_type = TextStyle::Primary;
    } else {
        style_type = style.unwrap();
    }
    let mut stylestring = "";
    match style_type {
        TextStyle::Primary => {
            stylestring = "font-sans text-lg";
        }
    }
    view! {
        <h3 class={stylestring}>{children()}</h3>
    }
}

#[component]
pub fn P(
    #[prop(optional)]
    style: Option<TextStyle>,
    children: Children,
) -> impl IntoView {
    let style_type;
    if style.is_none() {
        style_type = TextStyle::Primary;
    } else {
        style_type = style.unwrap();
    }
    let mut stylestring = "";
    match style_type {
        TextStyle::Primary => {
            stylestring = "font-serif text-base";
        }
    }
    view!{
        <p class={stylestring}>{children()}</p>
    }
}