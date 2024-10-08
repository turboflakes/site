use crate::components::buttons::Button;
use crate::components::sections::Section;
use crate::router::Routes;
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let navigator = navigator.clone();
        move |_| {
            navigator.push(&Routes::Home);
        }
    };

    html! {
        <Section>
            <h1>{"Page not found"}</h1>
            <p>
                {"Sorry, the page you're looking for could not be found."}
            </p>
            <div class="my-4">
                <Button label={"< back"} {onclick} />
            </div>
        </Section>
    }
}
