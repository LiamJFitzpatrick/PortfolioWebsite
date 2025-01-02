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
                <ul class="flex flex-row justify-between w-full">
                    <li class="border p-2"><a class="underline" href="#p-e-content">Professional Experience</a></li>
                    <li class="border p-2"><a class="underline" href="#proj-content">Projects</a></li>
                </ul>
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
                    "Improved tooling to assist in analyzing IQ data for understanding. The tool uses a Qt graphical user interface to allow the user to select which set of data to process and displays range-doppler maps for the analyst to review. I created support for an angle of arrival estimation algorithm, allowing the user to select a point of interest on the range-doppler map and view the estimated angle of arrival of that return. I created a config file that the tool stores its current state with, allowing the tool to remember what the analyst was doing when it was last open and saving the analyst time. I improved the binary file IQ parser that the tool had been using by supporting a design that only reads in header files to gather the necessary information to parse IQ binary until that specific set of data is needed, more of a just-in-time approach. Previously the tool would crash with a large dataset. I also implemented an LRU caching algorithm to allow for speedy recall of recent pieces of data.".into(),
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
                    "Worked at two different facilities after graduation at Ingredion. I was the Process Lead at a facility in Indianapolis, responsible for the daily operations of the Co-Products department. I was the Process Engineer (and would sometimes cover as Process Lead) in a facility in Kansas City, working across the Treating, Drying, & Packaging departments. A lot of this work was centered around people management and ensuring smooth daily operations.".into(),
                    "Provided data visualization tools to empower informed decision making during my time at facility and beyond. These tools allowed us to identify a bottleneck in the starch dewatering process. The Merco centrifuges were running 10 baume on underflow this was causing slow cycle times on Reinvelds. Not long after this and a few mechanical changes we were able to hit a new record of 2.05 million pounds of starch dried in one day.".into(),
                    "Led various smaller project installations in the under $500,000 range. Refurbished blend bin lid. Installed metal detector on line 2. Fixed and installed chemical line on LNP unit for HCl metering. Installed level detection equipment on various bins to provide greater insight into process health.".into(),
                    "Installed RF Gen barcoding system to allow for real time inventory tracking off production lines. Developed new standard procedures and trained with the new process. Developed project dashboards to display real time data to management and to line workers.".into()
                ] />
            
            <div class="my-16 mx-auto w-fit bg-[#0C120C]">
                <p>2021</p>
            </div>
            <ProfessionalExperienceCard
                img_path="/UMBC-primary-logo-RGB-2k.png".into()
                heading="Bachelor of Science in Chemical Engineering".into()
                subtitle="University of Maryland, Baltimore County".into()
                id="p-e-3".into()
                experience_blurbs=vec![
                    "Graduation with a 4.0/4.0 GPA from University of Maryland, Baltimore County. I studied Chemical Engineering.".into(),
                ] />
            
        </div>

        <H1>"Projects"</H1>
    }
}
