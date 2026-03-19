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
            // suno
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[384px] sm:w-[512px]" src="/img/suno_banner.webp" alt="suno" />
                </div>
                <p class="text-center">
                    <b>{"suno"}</b>{" — is a terminal user interface (TUI) to monitor live data and manage your own or third-party nodes. It supports Polkadot, Kusama, Paseo, and Westend networks. Checkout the project on GitHub."}
                    <GithubLink href="https://github.com/turboflakes/suno" />
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
                    <img class="w-[256px] sm:w-[512px]" src="/img/vals/polkadot_family.webp" alt="family of polkadot validators" />
                </div>
                <p class="text-center mb-8">
                    {"From left to right, "}<b>{"RAIDEN"}</b>{", "}<b>{"GOKUN"}</b>{", and "}<b>{"GALEN"}</b>{" are our high-performance, A+ Validators. "}<b>{"RAIDEN"}</b>{" has been securing the Polkadot network since 2021, while "}<b>{"GOKUN"}</b>{" and "}<b>{"GALEN"}</b>{" are proud members of the Decentralized Nodes Program, supported by the Web3 Foundation."} <VisitLink href="https://nodes.web3.foundation" label="program and rules." hidevisit={false} />
                </p>
                <p class="text-center mb-8">
                {"Together, they’re supercharged and ready to handle the most demanding workloads to keep the Polkadot network safe and secure. Staking rewards are triggered every era and you can track their performance through the links below."}
                </p>
                <div class="flex justify-center gap-4 mb-4">
                    <NominateCard address="12gPFmRqnsDhc9C5DuXyXBFA23io5fSGtKTSAimQtAWgueD2" chain="polkadot" identity="TURBOFLAKES.IO/RAIDEN" />
                    <NominateCard address="16BEvxYpyRWPaFbtwCPzSCtHVKr1soViaobKojNWBH12U5dk" chain="polkadot" identity="TURBOFLAKES.IO/GOKUN" />
                    <NominateCard address="145QiuMq8w1vBVXfwDVFxUSchdRZ6W1tTGb1uZ45TcoUskRC" chain="polkadot" identity="TURBOFLAKES.IO/GALEN" />
                </div>
            </Section>
            // Kusama Validators
            <Section>
                <div class="flex justify-center mb-16">
                    <img class="w-[256px] sm:w-[384px]" src="/img/vals/kusama_family.webp" alt="family of kusama validators" />
                </div>
                <p class="text-center mb-8">
                    {"Kusama's expected chaos and the fast-paced canary network are supported by the latest generation of processors used by this family of Validators. All our Kusama validators share the same 15% commission and on all you get instant rewards every era."}
                </p>
                <div class="flex flex-wrap justify-center gap-4 mb-4">
                    <NominateCard address="GA7j1FHWXpEU4kavowEte6LWR3NgZ8bkv4spWa9joiQF5R2" chain="kusama" identity="TURBOFLAKES.IO/MOMO"/>
                    <NominateCard address="FZsMKYHoQG1dAVhXBMyC7aYFYpASoBrrMYsAn1gJJUAueZX" chain="kusama" identity="TURBOFLAKES.IO/COCO"/>
                    <NominateCard address="FUu6iSzpfStHnbtbzFy2gsnBLttwNgNSULSCQCgMjPfkYwF" chain="kusama" identity="TURBOFLAKES.IO/DODO"/>
                    <NominateCard address="GwJweN3Q8VjBMkd2wWLQsgMXrwmFLD6ihfS146GkmiYg5gw" chain="kusama" identity="TURBOFLAKES.IO/TOTO"/>
                    <NominateCard address="HS4wfui3HrAG3K7UUFsUK4PVd1GXtqRQUdT5vH18gyTe88D" chain="kusama" identity="TURBOFLAKES.IO/JOJO"/>
                    <NominateCard address="Fm9FrPpsUZQvRRWgQMQHqdHvGPxq3qfwEyCMi8GqNH6tbEJ" chain="kusama" identity="TURBOFLAKES.IO/GOGO"/>
                    <NominateCard address="H2DRrJ1k3L4DyTGsCfhdo4mGNCf2fmx2X1XgePB9yqXvpvc" chain="kusama" identity="TURBOFLAKES.IO/BOBO"/>
                    <NominateCard address="G7HPomns9WnruzqbS3JH8gDfef8V1SsbrjQ7qnsckep8ct6" chain="kusama" identity="TURBOFLAKES.IO/LOLO"/>
                    <NominateCard address="HtR5DB7uwmQwU49p9XrFNKW35b2EaH8Ac7abAamyUYZyUvX" chain="kusama" identity="TURBOFLAKES.IO/FOFO"/>
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
