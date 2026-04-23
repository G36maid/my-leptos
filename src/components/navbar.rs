use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    let toggle_menu = move |_| {
        set_menu_open.update(|v| *v = !*v);
    };

    let close_menu = move |_| {
        set_menu_open.set(false);
    };

    view! {
        <nav class="navbar">
            <div class="navbar-container">
                <A href="/" attr:class="navbar-brand">
                    <span class="brand-icon">"*"</span>
                    <span class="brand-text">"Stellar"</span>
                </A>

                <div class="navbar-links" class:open=menu_open>
                    <A href="/" attr:class="nav-link" on:click=close_menu>
                        "Home"
                    </A>
                    <A href="/about" attr:class="nav-link" on:click=close_menu>
                        "About"
                    </A>
                    <A href="/projects" attr:class="nav-link" on:click=close_menu>
                        "Projects"
                    </A>
                    <A href="/contact" attr:class="nav-link" on:click=close_menu>
                        "Contact"
                    </A>
                </div>

                <button class="navbar-toggle" on:click=toggle_menu>
                    <span class="hamburger" class:open=menu_open>
                        <span></span>
                        <span></span>
                        <span></span>
                    </span>
                </button>
            </div>
        </nav>
    }
}
