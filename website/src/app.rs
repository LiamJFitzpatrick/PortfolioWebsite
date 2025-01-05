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
                experience_blurbs=vec![
                    "As a simulation engineer at Northrop Grumman's Emerging Capabilities division, I am currently developing a high-fidelity C++ simulation system that accurately simulates air-to-air radars. This innovative system connects the operational flight program to IQ data returns generated from the environment, pushing the boundaries of radar simulation technology and setting new standards in the field.".into(),
                    "Currently overseeing the software development process for this cutting-edge simulation requires strong technical expertise and leadership skills. By holding regular meetings and discussing updates from team members, I contribute to ensuring that the project progresses smoothly and efficiently while maintaining the highest standards of quality and innovation.".into(),
                    "I improved our analysis capabilities by enhancing an existing Qt graphical user interface tool. This updated tool allowed analysts to select specific data sets for processing and visualize range-doppler maps for thorough review. To further aid analysis, I implemented an angle of arrival estimation algorithm, enabling users to identify and visualize the estimated angle of arrival for areas of interest on the range-dopler map.".into(),
                    "Furthermore, I addressed the previous issue of tool crashes with large datasets by improving the binary file IQ parser. I introduced a just-in-time approach that only reads in header files to gather the necessary information to parse IQ binary until that specific set of data is needed. Additionally, I implemented an LRU caching algorithm to allow for speedy recall of recent pieces of data.".into(),
                    "To streamline analysis, I designed a configuration file that allowed the tool to remember and restore the user's previous session state, significantly reducing analysis time.".into(),
                    "My contributions extended beyond software development. I actively participated in the technical documentation process, playing a crucial role in Model Design Reviews (MDRs) at every stage of the project. This commitment to thorough documentation ensured that our work met the highest standards and was easily understood by all stakeholders.".into(),
                    "In addition to simulation development, I created PyCAM – a web application tailored for Control Account Managers (CAMs). This tool revolutionized cost tracking within programs, providing CAMs with real-time insights and enabling data-driven decision-making.".into(),
                    "Throughout my ongoing tenure at Northrop Grumman, I have demonstrated exceptional performance and earned the title of Top Performer during my first year. Since then, I consistently strive to maintain this high level of excellence in my work. Moreover, I take an active role in mentoring and training other engineers, fostering a culture of growth and knowledge sharing within the team. My dedication to excellence, leadership abilities, and commitment to innovation make me a valuable asset to the Emerging Capabilities division at Northrop Grumman, as I continue my work with the company.".into(),
                ] />
            <div class="my-16 mx-auto w-fit">
                <p>2023</p>
            </div>
            <ProfessionalExperienceCard
                img_path="/Ingredion_Logo_WebUseOnly_500.png".into()
                heading="Process Lead / Process Engineer".into()
                subtitle="Ingredion Incorporated".into()
                id="p-e-2".into()
                experience_blurbs=vec![
                    "After graduating, I embarked on a rewarding journey at Ingredion, working at two distinct facilities. In Indianapolis, I took on the role of Process Lead for the Co-Products department, overseeing the daily operations and ensuring seamless performance.".into(),
                    "My experience expanded further when I moved to Kansas City, where I served as the Process Engineer, occasionally stepping in as the Process Lead. Throughout my tenure, I worked diligently across the Treating, Drying, and Packaging departments, focusing on people management and maintaining optimal day-to-day processes.".into(),
                    "One of my significant contributions was developing data visualization tools that empowered informed decision-making not only within the facilities but also beyond. These innovative tools enabled us to identify a critical bottleneck in the starch dewatering process, where the Merco centrifuges were running 10 baume on underflow. This issue led to slow cycle times on Reinvelds. However, by implementing a few strategic mechanical changes, we achieved a new record of 2.05 million pounds of starch dried in a single day.".into(),
                    "Throughout my time at Ingredion, I spearheaded various smaller project installations, each with budgets under $500,000. These projects included refurbishing blend bin lids, installing metal detectors on line 2, fixing and installing the chemical line on the LNP unit for HCl metering, and implementing level detection equipment on various bins to provide real-time insights into process health.".into(),
                    "Moreover, I played a pivotal role in introducing the RF Gen barcoding system, which revolutionized inventory tracking by allowing for real-time monitoring off production lines. I also developed new standard procedures, trained employees with the new process, and created project dashboards to display real-time data to both management and line workers.".into(),
                    "My time at Ingredion was filled with challenges and achievements, providing me with valuable experience in people management, process optimization, and technological innovation within the food industry.".into(),
                ] />
            
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
