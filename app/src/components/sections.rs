use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div class="col-start-2 col-span-6 sm:col-start-3 sm:col-span-4">
            <div class="flex flex-col justify-center h-screen">
                {props.children.clone()}
            </div>
        </div>
    }
}
