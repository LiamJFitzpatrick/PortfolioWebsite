use leptos::prelude::*;
use super::super::components::text::{H2,H3,P};

pub enum CardStyle {
    Primary,
}


// Horizontal Card
#[component]
pub fn ProfessionalExperienceCard(
    #[prop(optional)]
    style: Option<CardStyle>,
    img_path: String,
    heading: String,
    subtitle: String,
    experience_blurbs: Vec<String>,
    id: String
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
    view!{
        <div id={id} class="border border-4 m-auto mt-4 rounded w-11/12 md:w-9/12 bg-[#0C120C] md:flex md:flex-row justify-around p-4">
            <div class="flex flex-col justify-center space-y-2 md:w-1/4">
                <img src={img_path} class="w-full" />
                <H2>{heading}</H2>
                <H3>{subtitle}</H3>
            </div>
            <div class="space-y-2 md:w-2/4">
                {
                    experience_blurbs.into_iter().map(|item| view!{
                        <P prop:innerHTML={item}>""</P>
                    }).collect_view()
                }
            </div>
        </div>
    }
}
    