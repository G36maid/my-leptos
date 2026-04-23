use leptos::{mount::mount_to_body, prelude::*};

mod app;
mod components;

use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
