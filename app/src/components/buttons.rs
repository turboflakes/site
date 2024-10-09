use yew::{classes, function_component, html, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: AttrValue,
    pub label: AttrValue,
    pub onclick: Callback<()>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| ());

    html! {
        <button class={classes!("btn", props.class.clone())} {onclick} disabled={props.disabled.clone()}>
            <span>{format!("{}", props.label.to_string())}</span>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct GithubLinkProps {
    pub href: AttrValue,
}

#[function_component(GithubLink)]
pub fn github_link(props: &GithubLinkProps) -> Html {
    html! {
        <span class="inline-flex items-baseline">
            <a href={props.href.clone()} target="_blank" class="ms-4">
                <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 .333A9.911 9.911 0 0 0 6.866 19.65c.5.092.678-.215.678-.477 0-.237-.01-1.017-.014-1.845-2.757.6-3.338-1.169-3.338-1.169a2.627 2.627 0 0 0-1.1-1.451c-.9-.615.07-.6.07-.6a2.084 2.084 0 0 1 1.518 1.021 2.11 2.11 0 0 0 2.884.823c.044-.503.268-.973.63-1.325-2.2-.25-4.516-1.1-4.516-4.9A3.832 3.832 0 0 1 4.7 7.068a3.56 3.56 0 0 1 .095-2.623s.832-.266 2.726 1.016a9.409 9.409 0 0 1 4.962 0c1.89-1.282 2.717-1.016 2.717-1.016.366.83.402 1.768.1 2.623a3.827 3.827 0 0 1 1.02 2.659c0 3.807-2.319 4.644-4.525 4.889a2.366 2.366 0 0 1 .673 1.834c0 1.326-.012 2.394-.012 2.72 0 .263.18.572.681.475A9.911 9.911 0 0 0 10 .333Z" clip-rule="evenodd"/>
                </svg>
            </a>
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct TwitterLinkProps {
    pub href: AttrValue,
}

#[function_component(TwitterLink)]
pub fn twitter_link(props: &TwitterLinkProps) -> Html {
    html! {
        <span class="inline-flex items-baseline">
            <a href={props.href.clone()} target="_blank" class="ms-4">
                <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                    <path fill-rule="evenodd" d="M22 5.892a8.178 8.178 0 0 1-2.355.635 4.074 4.074 0 0 0 1.8-2.235 8.343 8.343 0 0 1-2.605.981A4.13 4.13 0 0 0 15.85 4a4.068 4.068 0 0 0-4.1 4.038c0 .31.035.618.105.919A11.705 11.705 0 0 1 3.4 4.734a4.006 4.006 0 0 0 1.268 5.392 4.165 4.165 0 0 1-1.859-.5v.05A4.057 4.057 0 0 0 6.1 13.635a4.192 4.192 0 0 1-1.856.07 4.108 4.108 0 0 0 3.831 2.807A8.36 8.36 0 0 1 2 18.184 11.732 11.732 0 0 0 8.291 20 11.502 11.502 0 0 0 19.964 8.5c0-.177 0-.349-.012-.523A8.143 8.143 0 0 0 22 5.892Z" clip-rule="evenodd"/>
                </svg>
            </a>
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct VisitLinkProps {
    pub href: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub hidevisit: bool,
}

#[function_component(VisitLink)]
pub fn visit_link(props: &VisitLinkProps) -> Html {
    html! {
        <span class="ms-2 inline-flex items-baseline">
            { if !props.hidevisit {
                html! { <p>{"Visit"}</p> }
                } else {
                    html! {}
                }
            }
            <a href={props.href.clone()} target="_blank" class="flex hover:underline hover:underline-offset-2">
                <span class="mx-2">{props.label.clone()}</span>
                <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 14v4.833A1.166 1.166 0 0 1 16.833 20H5.167A1.167 1.167 0 0 1 4 18.833V7.167A1.166 1.166 0 0 1 5.167 6h4.618m4.447-2H20v5.768m-7.889 2.121 7.778-7.778"/>
                </svg>
            </a>
        </span>
    }
}
