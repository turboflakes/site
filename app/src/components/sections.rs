use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div class="col-span-8 sm:col-start-2 sm:col-span-6 md:col-start-3 md:col-span-4 mx-2 sm:mx-0">
            <div class="flex flex-col justify-center h-screen">
                {props.children.clone()}
            </div>
        </div>
    }
}
