use crate::components::icons::Identicon;
use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct NominateCardProps {
    // address is an ss58-encoded address or publicKey
    pub address: AttrValue,
    // identity is the display name
    pub identity: AttrValue,
}

#[function_component(NominateCard)]
pub fn nominate(props: &NominateCardProps) -> Html {
    html! {
        <div class="inline-flex items-center min-w-[190px]">
            <Identicon address={props.address.to_string()} size={32} class="me-2" />
            <div class="flex flex-col">
                <span class="font-mono text-xs text-left">{compact(props.address.to_string())}</span>
                <span class="font-mono text-xs text-left">{props.identity.clone()}</span>
            </div>
        </div>
    }
}

pub fn compact(address: String) -> String {
    [&address[..6], &address[address.len() - 6..address.len()]].join("...")
}
