use leptos::prelude::*;

pub enum ButtonStyle {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Cyberpunk
}

#[component]
pub fn Button(
    #[prop(optional)]
    style: Option<ButtonStyle>,
    children: Children,
) -> impl IntoView {
    let style_type;
    if style.is_none() {
        style_type = ButtonStyle::Primary;
    } else {
        style_type = style.unwrap();
    }
    let mut stylestring = "";
    match style_type {
        ButtonStyle::Primary => {
            stylestring = "transition ease-in-out delay-150 hover:scale-110 border-2 py-2 px-4 rounded";
        },
        ButtonStyle::Secondary => {
            stylestring = "bg-gray-300 hover:bg-gray-500 text-black font-semibold py-2 px-4 rounded";
        },
        ButtonStyle::Success => {
            stylestring = "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded";
        }, 
        ButtonStyle::Danger => {
            stylestring = "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded";
        },
        ButtonStyle::Warning => {
            stylestring = "bg-yellow-500 hover:bg-yellow-700 text-black font-semibold py-2 px-4 rounded";
        },
        ButtonStyle::Cyberpunk => {
            stylestring = "bg-[#0a0c1b] hover:bg-[#171d3a] text-white font-bold py-2 px-4 rounded shadow-lg transition duration-300 ease-in-out transform hover:scale-105";
        }
    }
    view! {
        <button class={stylestring}>{children()}</button>
    }
}