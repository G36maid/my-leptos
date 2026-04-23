use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::{About, Contact, Home, Navbar, NotFound, Projects};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/my-leptos">
            <div class="app">
                <Navbar/>
                <main class="main-content">
                    <Routes fallback=|| view! { <NotFound/> }>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                        <Route path=path!("/projects") view=Projects/>
                        <Route path=path!("/contact") view=Contact/>
                    </Routes>
                </main>
                <footer class="footer">
                    <div class="footer-content">
                        <div class="footer-brand">
                            <span class="footer-logo">"Stellar"</span>
                            <p class="footer-tagline">"Crafted with Leptos & Rust"</p>
                        </div>
                        <div class="footer-links">
                            <div class="footer-col">
                                <h4>"Navigate"</h4>
                                <a href="/">"Home"</a>
                                <a href="/about">"About"</a>
                                <a href="/projects">"Projects"</a>
                                <a href="/contact">"Contact"</a>
                            </div>
                            <div class="footer-col">
                                <h4>"Connect"</h4>
                                <a href="#">"GitHub"</a>
                                <a href="#">"Twitter"</a>
                                <a href="#">"Discord"</a>
                            </div>
                        </div>
                    </div>
                    <div class="footer-bottom">
                        <p>"© 2026 Stellar. All rights reserved."</p>
                    </div>
                </footer>
            </div>
        </Router>
    }
}
