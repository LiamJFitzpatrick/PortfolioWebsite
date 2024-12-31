use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, path, StaticSegment
};
use leptos_components::components::buttons::{Button, ButtonStyle};
use leptos_components::components::text::{H1, H2, H3, P};

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
            
            <div id="p-e-1" class="border border-4 m-auto mt-4 rounded w-1/2 h-fit bg-[#0C120C]">
                <div class="ml-8">
                    <H2>"Modeling Simulation & Analysis Engineer"</H2>
                    <H3>"Northrop Grumman Corporation"</H3>
                    <P>plop</P>
                </div>
            </div>
            <div class="my-16 mx-auto w-fit bg-[#0C120C]">
                <p>2023</p>
            </div>
            <div id="p-e-2" class="border border-4 m-auto rounded w-3/5 h-32 bg-[#0C120C]">
            </div>
            <div class="my-16 mx-auto w-fit bg-[#0C120C]">
                <p>2021</p>
            </div>
            <div id="p-e-3" class="border border-4 m-auto rounded w-3/5 h-32 bg-[#0C120C]">
                <H2>Graduation</H2>
            </div>
        </div>
    }
}
