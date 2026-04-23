use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="page page-not-found">
            <div class="not-found-content">
                <h1 class="not-found-code">"404"</h1>
                <p class="not-found-text">"Page not found"</p>
                <A href="/" attr:class="btn btn-primary">
                    "Go Home"
                    <span class="btn-arrow">"→"</span>
                </A>
            </div>
        </div>
    }
}
