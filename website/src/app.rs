use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, path, StaticSegment
};
use leptos_components::components::buttons::{Button, ButtonStyle};
use leptos_components::components::text::{H1, H2, H3, P};
use leptos_components::elements::cards::ProfessionalExperienceCard;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        <Stylesheet href="/static_css.css" />

        <Script src="/profesh-exp.js" />

        // sets the document title
        <Title text="My Portfolio"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    let mut all_northrop_exp: Vec<String> = vec![];
    let mut all_ingredion_exp: Vec<String> = vec![];

    for section in include_str!("../inputs/northrop.md").to_string().split("\n") {
        if section != "" {
            all_northrop_exp.push(section.to_string());
        }
    }

    for section in include_str!("../inputs/ingredion.md").to_string().split("\n") {
        if section != "" {
            all_ingredion_exp.push(section.to_string());
        }
    }

    view! {
        <script type="module" src="/load_bevy_anim.js">
        </script>
        <canvas id="bevy-canvas1" width="100vw"></canvas>
        <div id="hero" class="md:flex md:flex-row min-h-screen md:justify-center md:items-center">
            <img width="320px" class="rounded" src="/00114-3885650054.png" />
            <div class="flex flex-col m-4 p-2 items-start max-w-lg">
                <ul class="flex flex-row justify-around w-full">
                    <li class="border p-2"><a class="underline" href="#p-e-content">Professional Experience</a></li>
                    <li class="border p-2"><a class="underline" href="#proj-content">Projects</a></li>
                    <li class="border p-2"><a class="underline" href="https://github.com/LiamJFitzpatrick/PortfolioWebsite">Source</a></li>
                </ul>
                <H1>Hi!</H1>
                <H2>"I'm Liam Fitzpatrick."</H2>
                <P>"I'm an engineer with expertise in programming, simulation, and process improvement. I have a proven ability 
                to develop innovative solutions, optimize complex systems, and lead successful projects. I have some of my experiences 
                outlined on this website. If you want to hire me or ask questions about my projects, reach out to me "<a class="underline" href="mailto:liam.fitzpatrick@live.com">here</a>.</P>
            </div>
        </div>

        <canvas id="p-e-canvas"></canvas>
        

        <div id="p-e-content" class="min-h-screen">
            <H1>"Professional Experience"</H1>
            
            <ProfessionalExperienceCard
                img_path="/NGC-logo-white-on-clear.webp".into()
                heading="Modeling Simulation & Analysis Engineer".into()
                subtitle="Northrop Grumman Corporation".into()
                id="p-e-1".into()
                experience_blurbs=all_northrop_exp />
            <div class="my-16 mx-auto w-fit">
                <p>2023</p>
            </div>
            <ProfessionalExperienceCard
                img_path="/Ingredion_Logo_WebUseOnly_500.png".into()
                heading="Process Lead / Process Engineer".into()
                subtitle="Ingredion Incorporated".into()
                id="p-e-2".into()
                experience_blurbs=all_ingredion_exp />
            
            <div class="my-16 mx-auto w-fit">
                <p>2021</p>
            </div>
            <ProfessionalExperienceCard
                img_path="/UMBCretrievers_LOGO.jpg".into()
                heading="Bachelor of Science in Chemical Engineering".into()
                subtitle="University of Maryland, Baltimore County".into()
                id="p-e-3".into()
                experience_blurbs=vec![
                    "Graduation with a 4.0/4.0 GPA from University of Maryland, Baltimore County. I studied Chemical Engineering.".into(),
                ] />
            
        </div>

        
        <div id="proj-content" class="min-h-screen">
            <H1>"Projects"</H1>
            <ProfessionalExperienceCard
                img_path="/strategyeats_logo.png".into()
                heading="StrategyEats.com".into()
                subtitle="Founder & President".into()
                id="proj-1".into()
                experience_blurbs=vec![
                    "Meal delivery platform with the mission to help combat the challenges that Americans face when it comes to eating. I want eating healthy to be an enjoyable and easy thing for all of us. StrategyEats.com provides ready-to-eat meals delivered to your doorstep, through an intuitive website interface.".into(),
                    "I was able to achieve over $2k in revenue in second month of accepting orders. Received great positive feedback.".into(),
                    "I developed the platform using the Leptos framework, a full-stack web framework written in Rust. The platform has an integrated resource planning, recipe planning, and point of sale system, allowing for streamlined operations between the customer and production.".into(),
                    "Created the business model and business plan. ".into(),
                    "Check it out now by visiting <a href=\"https://www.strategyeats.com\">https://www.strategyeats.com</a>.".into(),
                ] />
            <ProfessionalExperienceCard 
                img_path="/strategylifts_logo.png".into()
                heading="StrategyLifts.com".into()
                subtitle="Sole Developer".into()
                id="proj-2".into()
                experience_blurbs=vec![
                    "Workout tracking platform designed for those who like to look at data! Website tracks progression and suggests workout plans to assist user into progressive overload. Allows for sharing workouts, tracking progress, and getting recommended workouts. ".into(),
                    "This website utilizes the Django framework, written in Python. The site is currently free for anyone to use. I don't have any current plans to monetize the project, I currently use it myself to track my workouts and let anyone else do the same!".into(),
                    "Check it out now by visiting <a href=\"https://www.strategylifts.com\">https://www.strategylifts.com</a>.".into(),
                ] />

            <ProfessionalExperienceCard
                img_path="/00112-3953999397.png".into()
                heading="CapitalCompass".into()
                subtitle="Sole Developer".into()
                id="proj-3".into()
                experience_blurbs=vec![
                    "Financial tracking application. Current targeted features are tracking spending and financial projections. The spending tracking is geared towards a method that me and my wife use for tracking our bills. The financial projections are designed for simulating business scenarios to determine profitability. ".into(),
                    "This tool is still a work in progress. I am building it as a local application but using Tauri to develop it. Tauri is similar to Electron for JavaScript applications but it is written in Rust.".into()
                ] />
        </div>
    }
}
