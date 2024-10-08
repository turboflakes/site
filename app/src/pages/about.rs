use crate::components::{buttons::Button, sections::Section};
use crate::router::Routes;
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

#[function_component(AboutPage)]
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
            <h1>{"About us"}</h1>
            <p>
                {"Our team brings a diverse range of professional experience, spanning Electronics and Computer Science engineering to Creative Graphic Design, with all of us sharing a passion for blockchain technology. Some of us have developed various applications across industries such as healthcare and finance, utilizing different kinds of technologies. One team member is a veteran in the gaming industry as co-founder of a game publishing company, while another has worked as an Art Director at top advertising agencies."}
            </p>
            <p>
                {"We are self-funded and independent, actively validating on the Kusama network since January 2021 and on Polkadot since June 2021. We've been part of the Polkadot Validator community since the network crossed our path and are active participants in the One Thousand Validators Program, led by the Web3 Foundation and Parity Technologies."}
            </p>
            <p>
                {"With our extensive IT professional background, we have the expertise and confidence to make our top-notch hybrid infrastructure as reliable as possible. This ensures that Nominators can trust and feel confident by nominating our validators."}
            </p>
            <p>
                {"Beyond validation and providing blockchain infrastructure, our roots and passion lies in experimentation, open-source innovation, and creation. We often take a random thought, need, or idea and work tirelessly to bring it to life."}
            </p>
            <p>
                {"We are also careful listeners, if you have ideas, feature requests or are interested in a potential collaboration, feel free to reach out to us at "}
                <a href="mailto:hey@turboflakes.io">{"hey@turboflakes.io"}</a>{"."}
            </p>
            <div class="my-4">
                <Button label={"< back"} {onclick} />
            </div>
        </Section>
    }
}
