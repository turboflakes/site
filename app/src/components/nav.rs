use crate::components::buttons::{GithubLink, TwitterLink};
use yew::{function_component, html, Html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="fixed w-full z-20 top-0 start-0 bg-gradient-to-r from-white from-20%">
            <div class="flex items-center justify-between px-2 sm:p-4 h-12">
                <a class="block" href="https://turboflakes.io/">
                    <img class="h-8" src="/images/logo_turboflakes_r2.svg" alt="turboflakes logo" />
                </a>
            </div>
        </nav>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="grid grid-cols-8 gap-4">
            <footer class="col-start-3 col-span-4">
                <div class="flex justify-center my-4">
                    <span class="me-1">{"© 2024 turboflakes.io — Built by"}</span>
                    <a href="https://turboflakes.io/" target="_blank" class="flex items-center hover:underline hover:underline-offset-2">
                        <span class="me-4">{"Turboflakes"}</span>
                        <svg class="w-4 h-4" viewBox="0 0 60 60" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                            <circle fill-rule="evenodd" cx="30" cy="30" r="30"></circle>
                            <path d="M20,46 C17.2399393,46 15,43.7600607 15,41 C15,38.2399393 17.2399393,36 20,36 C22.7600607,36 25,38.2399393 25,41 C25,43.7600607 22.7600607,46 20,46 Z M32.0751821,35 C29.3566227,35 18,30 18,30 C18,30 29.3566227,25 32.0751821,25 C34.7937414,25 37,27.2399393 37,30 C37,32.7600607 34.7974808,35 32.0751821,35 Z M39.777954,24 C36.8953212,24 20,19 20,19 C20,19 36.8913561,14 39.777954,14 C42.6645519,14 45,16.2399393 45,19 C45,21.7600607 42.6645519,24 39.777954,24 Z" fill="#EAEDF0"></path>
                        </svg>
                    </a>
                    <GithubLink href="https://github.com/turboflakes/turboflakes.github.io" />
                    <TwitterLink href="https://x.com/turboflakes" />


                    <a href="/about" class="ms-4">
                        {"About"}
                    </a>
                    <a href="/privacy" class="ms-4">
                        {"Privacy"}
                    </a>
                    <a href="/terms" class="ms-4">
                        {"Terms"}
                    </a>
                </div>
            </footer>
        </div>
    }
}
