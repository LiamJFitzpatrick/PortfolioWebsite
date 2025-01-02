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
    view! {
        <canvas id="p-e-canvas"></canvas>
        <div class="flex flex-row min-h-screen justify-center items-center">
            <img class="max-w-lg rounded" src="/founder_portrait.jpg" />
            <div class="flex flex-col m-4 p-2 items-start max-w-lg">
                <H1>Hi!</H1>
                <H2>"I'm Liam Fitzpatrick."</H2>
                <P>"I'm an engineer with expertise in modeling, simulation, and process improvement. I have a proven ability 
                to develop innovative solutions, optimize complex systems, and lead successful projects. I have some of my experiences 
                projects outlined on this website. If you want to hire me or ask questions about my projects reach out to me "<a class="underline" href="mailto:liam.fitzpatrick@live.com">here</a>.</P>
            </div>
        </div>
        <H1>"Professional Experience"</H1>

        <div id="p-e-content" class="min-h-screen">
            
            <ProfessionalExperienceCard
                img_path="/NGC-logo-white-on-clear.webp".into()
                heading="Modeling Simulation & Analysis Engineer".into()
                subtitle="Northrop Grumman Corporation".into()
                id="p-e-1".into()
                experience_blurbs=vec![
                    "Developed C++ simulation for high fidelity simulation of air-to-air radars for Emerging Capabilities division. The system simulates from the environment to IQ data returns connected to operational flight program. ".into(),
                    "Lead software development process of C++ simulation for a team of 6 engineers.".into(),
                    "Created tooling to assist in analyzing IQ data for understanding the angle of arrival, range, and velocity of return. The tool uses a Qt graphical user interface to allow the user to select which set of data to pr".into(),
                    "Contributed to technical documentation process – Model Design Review (MDR) – at all stages.".into(),
                    "Developed a web application – PyCAM – to assist control account managers in tracking costs within programs.".into(),
                    "Additionally: recognized as Top Performer, mentored & trained other engineers".into(),
                ] />
            <div class="my-16 mx-auto w-fit bg-[#0C120C]">
                <p>2023</p>
            </div>
            <ProfessionalExperienceCard
                img_path="/Ingredion_Logo_WebUseOnly_500.png".into()
                heading="Process Lead / Process Engineer".into()
                subtitle="Ingredion Incorporated".into()
                id="p-e-2".into()
                experience_blurbs=vec![
                    "Provided data visualization tools to empower informed decision making during my time at facility and beyond. These tools allowed us to identify a bottleneck in the starch dewatering process. The Merco centrifuges were running 10 baume on underflow this was causing slow cycle times on Reinvelds. Not long after this and a few mechanical changes we were able to hit a new record of 2.05 million pounds of starch dried in one day.".into(),
                    "Managed contractors on project installs. Refurbished blend bin lid. Installed metal detector on line 2. Fixed and installed chemical line on LNP unit for HCl metering. Installed level detection equipment on various bins to provide greater insight into process health.".into(),
                    "Installed RF Gen barcoding system to allow for real time inventory tracking off production lines. Developed new standard procedures and trained with the new process. Developed project dashboards to display real time data to management and to line workers.".into()
                ] />
            
            <div class="my-16 mx-auto w-fit bg-[#0C120C]">
                <p>2021</p>
            </div>
            <div id="p-e-3" class="border border-4 m-auto rounded w-3/5 h-32 bg-[#0C120C]">
                <H2>Graduation</H2>
            </div>
        </div>
    }
}
