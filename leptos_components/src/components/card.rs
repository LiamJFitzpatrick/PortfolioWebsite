use leptos::prelude::*;

pub enum CardStyle {
    Primary,
}


// Horizontal Card
#[component]
pub fn HCard<'a>(
    #[prop(optional)]
    style: Option<CardStyle>,
    img_path: &'a str,

) -> impl IntoView {
    let style_type;
    if style.is_none() {
        style_type = CardStyle::Primary;
    } else {
        style_type = style.unwrap();
    }
    let mut stylestring = "";
    match style_type {
        CardStyle::Primary => {
            stylestring = "font-sans text-2xl";
        }
    }
    view! {
    }
}