use crate::components::{buttons::Button, sections::Section};
use crate::router::Routes;
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

#[function_component(PrivacyPage)]
pub fn page() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let navigator = navigator.clone();
        move |_| {
            navigator.push(&Routes::Home);
        }
    };

    html! {
        <Section>
            <time>{"Last updated: October 08, 2024"}</time>
            <h1>{"Privacy Policy"}</h1>
            <p>
                {"The privacy of our website visitors is important to us so we do not track any individual people. As a visitor to the turboflakes.io, apps.turboflakes.io (or any other subdomain), goclaimit.app, corematch.io listed in this website:"}
            </p>
            <ul>
                <li>{"No personal information is collected"}</li>
                <li>{"No information such as cookies is stored in the browser"}</li>
                <li>{"No information is shared with, sent to or sold to third-parties"}</li>
                <li>{"No information is shared with advertising companies"}</li>
                <li>{"No information is mined and harvested for personal and behavioral trends"}</li>
                <li>{"No information is monetized"}</li>
            </ul>
            <p>
                {"We run the "}
                <span class="inline-flex items-baseline">
                    <a href="https://plausible.io/docs/" class="underline">{"Plausible Analytics"}</a>
                </span>
                {" script to collect some anonymous usage data for statistical purposes. The goal is to track overall trends in our website traffic, it is not to track individual visitors. All the data is in aggregate only. No personal data is collected."}
            </p>
            <p>{"Data collected includes referral sources, top pages, visit duration, information from the devices (device type, operating system, country and browser) used during the visit and more. You can see full details in Plausible Analytics "}
                <span class="inline-flex items-baseline">
                    <a href="https://plausible.io/data-policy" class="underline">{"data policy"}</a>{"."}
                </span>
            </p>
            <h4>{"Contact Us"}</h4>
            <p>{"If you have a question about this Privacy Policy, please contact us at "}<a href="mailto:privacy@turboflakes.io">{"privacy@turboflakes.io"}</a>{"."}</p>

            <div class="my-4">
                <Button label={"< back"} {onclick} />
            </div>
        </Section>
    }
}
