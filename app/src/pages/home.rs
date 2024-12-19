use crate::components::{
    buttons::{GithubLink, VisitLink},
    cards::NominateCard,
    sections::Section,
};
use yew::{function_component, html, Html};

#[function_component(HomePage)]
pub fn home() -> Html {
    html! {
        <>
            <Section>
                <p class="mb-6">
                    <b>
                    {"Embrace Web3 and the Open-Source Movement!"}
                    </b>
                </p>
                <p class="mb-6">
                    {"On this site, you'll find a compilation of ideas.. including work-in-progress utilities, tools, bots, visual identities, digital experiences, decentralized applications (dApps), and services — all built on top of open-source technology for the Polkadot ecosystem. Each is a small but meaningful contribution to the open and decentralized web values we proudly champion and remain committed to."}
                </p>
                <p class="mb-6">
                    {"These tools are designed to help and simplify interactions with Substrate-based blockchain networks, enhance understanding of key components of the Polkadot protocol, and uncover patterns through visualization. We make them up as we go, with no long-term plans. Some are built to address challenges we stumble upon, while others are created as a way to explore and learn new tech, and by sharing them, we hope others find them useful and make the most out of them."}
                </p>
                <p>
                    {"Join us on our Web3 journey and exploration by"}
                    <span class="inline-flex items-baseline ms-2">
                        <a class="block" href="https://turboflakes.io/">
                            <img class="h-6 inline-block" src="/img/logo/logo_turboflakes_r2.svg" alt="turboflakes logo" />
                        </a>
                    </span>
                </p>
            </Section>
            // claim.it
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[128px] sm:w-[192px]" src="/img/claimit_logo.svg" alt="claim.it" />
                </div>
                <p class="text-center">
                    <b>{"claim.it"}</b>{" — is a decentralized application (dApp) with a simple user interface, making it easy to look up and claim child bounties. The tool is designed for beneficiary users, allowing them to follow and claim their favorite child bounty awards. You can try it out at"} 
                    <VisitLink href="https://goclaimit.app/" hidevisit={true} label="goclaimit.app" />
                    <GithubLink href="https://github.com/turboflakes/claimit" />
                </p>
            </Section>
            // corematch
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[256px] sm:w-[384px]" src="/img/corematch_logo.svg" alt="corematch" />
                </div>
                <p class="text-center">
                    <b>{"COREMATCH"}</b>{" — is an engaging memory game where players match the latest Polkadot core usage in a 3x3 matrix board. The game is designed to highlight the randomness in the multi-core protocol implementation of the Polkadot network. You can play it at"}
                    <VisitLink href="https://corematch.xyz/" hidevisit={true} label="corematch.xyz" />
                    <GithubLink href="https://github.com/turboflakes/corematch" />
                </p>
            </Section>
            // one-t
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[128px] sm:w-[192px]" src="/img/onet.svg" alt="one-t" />
                </div>
                <p class="text-center">
                    <b>{"ONE-T"}</b>{" — is an indexer and the backbone of a unique analytics dashboard for the Polkadot, Kusama, and Paseo networks. It also serves as a performance report Matrix Bot, Nominator, and Curator bot for "}<i>{"Turboflakes Nomination Pools."}</i>
                </p>
                <p class="text-center">
                    <VisitLink href="https://apps.turboflakes.io/#/dashboard" label="apps.turboflakes.io" />
                    <GithubLink href="https://github.com/turboflakes/one-t" />
                    <GithubLink href="https://github.com/turboflakes/apps" />
                </p>
            </Section>
            // crunch
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[128px] sm:w-[200px]" src="/img/crunchbot.svg" alt="crunchbot" />
                </div>
                <p class="text-center">
                    <b>{"CRUNCH"}</b>{" — is a command-line interface (CLI) and Matrix Bot to easily automate payouts of staking rewards. Allows stakers to get notified about the amount and rate of the total staking rewards each Validator and their Nominators got and is also a handy tool for Pools Operators to auto-compound members rewards."}
                </p>
                <p class="text-center">
                    <GithubLink href="https://github.com/turboflakes/crunch" />
                </p>
            </Section>
            // nomi
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[192px] sm:w-[256px]" src="/img/nomi.svg" alt="nomi" />
                </div>
                <p class="text-center">
                    <b>{"NOMI"}</b>{" — is a visual experience tool designed to actively involve Nominators in Native Staking. It aims to offer a unique and enhanced nomination experience, using Multiple-Criteria Decision Analysis as base for the analytical research."}
                </p>
                <p class="text-center">
                    <VisitLink href="https://apps.turboflakes.io/?chain=polkadot&app=nomi#/dashboard" label="apps.turboflakes.io" />
                    <GithubLink href="https://github.com/turboflakes/apps" />
                </p>
            </Section>
            // scouty
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[128px] sm:w-[192px] sm:max-w-[256px]" src="/img/scouty.svg" alt="scoutybot" />
                </div>
                <p class="text-center">
                    <b>{"SCOUTY"}</b>{" — is a command-line interface (CLI) to keep an eye on substrate-based chains and hook things up. Scouty mainly purpose is to monitor, intercept and extend functionality as soon as on-chain events are emitted."}
                </p>
                <p class="text-center">
                    <GithubLink href="https://github.com/turboflakes/scouty" />
                </p>
            </Section>
            // Polkadot Validators
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[256px] sm:w-[320px]" src="/img/vals/raiden.svg" alt="raiden" />
                </div>
                <p class="text-center mb-8">
                    <b>{"RAIDEN"}</b>{" — is a high-performance, A+ Validator that has been securing the Polkadot network since 2021. Raiden's commission is 1% and rewards are triggered every era."}
                </p>
                <div class="flex justify-center">
                    <NominateCard address="12gPFmRqnsDhc9C5DuXyXBFA23io5fSGtKTSAimQtAWgueD2" chain="polkadot" identity="TURBOFLAKES.IO/RAIDEN" />
                </div>
            </Section>
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[256px] sm:w-[320px]" src="/img/vals/gokun.svg" alt="gokun" />
                </div>
                <p class="text-center mb-8">
                    <b>{"GOKUN"}</b>{" — is a supercomputer Validator warrior ready to handle demanding workloads to secure the Polkadot network. Gokun's commission is 3%. Similar to Raiden, rewards are guaranteed by CRUNCH at the end of each era."}
                </p>
                <div class="flex justify-center">
                    <NominateCard address="16BEvxYpyRWPaFbtwCPzSCtHVKr1soViaobKojNWBH12U5dk" chain="polkadot" identity="TURBOFLAKES.IO/GOKUN"/>
                </div>
            </Section>
            // Kusama Validators
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[256px] sm:w-[384px]" src="/img/vals/kusama_family.svg" alt="family of kusama validators" />
                </div>
                <p class="text-center mb-8">
                    {"Kusama's expected chaos and the fast-paced canary network are supported by the latest generation of processors used by this family of Validators. All our Kusama validators share the same 10% commission and on all you get instant rewards every era."}
                </p>
                <div class="flex flex-wrap justify-center gap-4 mb-4">
                    <NominateCard address="GA7j1FHWXpEU4kavowEte6LWR3NgZ8bkv4spWa9joiQF5R2" chain="kusama" identity="TURBOFLAKES.IO/MOMO"/>
                    <NominateCard address="FZsMKYHoQG1dAVhXBMyC7aYFYpASoBrrMYsAn1gJJUAueZX" chain="kusama" identity="TURBOFLAKES.IO/COCO"/>
                    <NominateCard address="FUu6iSzpfStHnbtbzFy2gsnBLttwNgNSULSCQCgMjPfkYwF" chain="kusama" identity="TURBOFLAKES.IO/DODO"/>
                    <NominateCard address="GwJweN3Q8VjBMkd2wWLQsgMXrwmFLD6ihfS146GkmiYg5gw" chain="kusama" identity="TURBOFLAKES.IO/TOTO"/>
                    <NominateCard address="HS4wfui3HrAG3K7UUFsUK4PVd1GXtqRQUdT5vH18gyTe88D" chain="kusama" identity="TURBOFLAKES.IO/JOJO"/>
                    <NominateCard address="Fm9FrPpsUZQvRRWgQMQHqdHvGPxq3qfwEyCMi8GqNH6tbEJ" chain="kusama" identity="TURBOFLAKES.IO/GOGO"/>
                </div>
            </Section>
            <Section>
                <p class="mb-6">
                    {"Beyond the projects and Validators mentioned above, we also provide RPC and Collators infrastructure for Polkadot and Kusama."}
                </p>
                <p class="mb-6">
                    {"We hope you enjoy and find some of these projects useful. And we wish it could inspire fellow tinkerers and builders out there. If for any of these projects you have an idea, a feature request, a fix or you found a bug, we would be really grateful if you could let us know or even open an issue in the project repo - we will do our best to fix it or make your request available."}
                </p>
                <p class="mb-6">
                    {"You can also give us a follow, consider joining our nomination pools or nominate one of our validators. Your backing makes a difference!"}
                </p>
                <p>
                    {"Thanks for reading"}
                    <span class="inline-flex items-baseline ms-2">
                        <a class="block" href="https://turboflakes.io/">
                            <img class="h-6 inline-block" src="/img/logo/logo_turboflakes_r2.svg" alt="turboflakes logo" />
                        </a>
                    </span>
                </p>
            </Section>
        </>
    }
}
