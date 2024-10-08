use crate::components::{
    buttons::{Button, VisitLink},
    sections::Section,
};
use crate::router::Routes;
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

#[function_component(TermsPage)]
pub fn page() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let navigator = navigator.clone();
        move |_| {
            navigator.push(&Routes::Home);
        }
    };

    html! {
        <div class="grid grid-cols-8 gap-4">
            <Section>
                <time>{"Last updated: October 08, 2024"}</time>
                <h1>{"Terms of use"}</h1>
                <h4>
                    {"Tools, Utilities, Bots, Apps and dApps"}
                </h4>
                <p>
                    {"All projects featured on this website (claim.it, COREMATCH, ONE-T, NOMI, CRUNCH, SCOUTY) are provided “AS IS”, without warranty of any kind, all are open-source, licensed under the Apache License, Version 2.0 or MIT. The authors are not liable for any damages or any loss of funds resulting from the use of these projects."}
                    <VisitLink href="http://www.apache.org/licenses/LICENSE-2.0" label="Apache License, Version 2.0" />
                </p>
                <h4>{"Validators"}</h4>
                <p>
                    {"Our validators provide a non-custodial staking service, meaning we do not have access to your staked or unstaked tokens."}
                </p>
                <p>
                    {"We rarely change our validator commission. However, if we ever decide to do so, we will announce it here on our website first. Please check back regularly for updates to stay informed of any changes."}
                </p>
                <p>
                    {"Our validators are monitored 24/7 using well-known tools like Prometheus, Grafana, Grafana On-Call, along with our custom monitoring solutions, ONE-T and SCOUTY. Our validators have never been slashed, and we respond quickly to the best of our ability to any infrastructure downtime that may arise."}
                </p>
                <p>
                    {"By nominating our validators, you understand and acknowledge and accept the following:"}
                </p>
                <ul>
                    <li>{"You are well informed about how Nominated Proof of Staking (NPoS) works, slashing, and the inherent risks of losing your tokens due to slashing events."}</li>
                    <li>{"You have done your own research when selecting the validators you nominate, and you may not earn staking rewards if the validators experience downtime. You are free to end your staking with us at any time."}</li>
                    <li>{"We are not liable for any damages resulting from your use of our services."}</li>
                    <li>{"We reserve the right to increase the validator commission or to stop operations of the validators immediately and indefinitely."}</li>
                </ul>
                <p>
                    {"We reserve the right of updating these terms of use at any given time."}
                </p>
                <div class="my-4">
                    <Button label={"< back"} {onclick} />
                </div>
            </Section>
        </div>
    }
}
