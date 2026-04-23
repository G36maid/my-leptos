use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (submitted, set_submitted) = signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !name.get().is_empty() && !email.get().is_empty() && !message.get().is_empty() {
            set_submitted.set(true);
        }
    };

    view! {
        <div class="page page-contact">
            <section class="section contact-hero">
                <span class="section-tag">"Get In Touch"</span>
                <h1 class="contact-title">
                    "Let's "
                    <span class="text-gradient">"Connect"</span>
                </h1>
                <p class="contact-intro">
                    "Have a project in mind? Want to collaborate? We'd love to hear from you."
                </p>
            </section>

            <section class="section contact-section">
                <div class="contact-grid">
                    <div class="contact-form-wrapper">
                        {move || if submitted.get() {
                            view! {
                                <div class="success-card">
                                    <div class="success-icon">"✓"</div>
                                    <h3>"Message Sent!"</h3>
                                    <p>"Thank you for reaching out. We'll get back to you within 24 hours."</p>
                                    <button
                                        class="btn btn-ghost"
                                        on:click=move |_| set_submitted.set(false)
                                    >
                                        "Send Another"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <form class="contact-form" on:submit=on_submit>
                                    <div class="form-group">
                                        <label class="form-label">"Name"</label>
                                        <input
                                            type="text"
                                            class="form-input"
                                            placeholder="Your name"
                                            prop:value=name
                                            on:input=move |ev| set_name.set(event_target_value(&ev))
                                            required
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label class="form-label">"Email"</label>
                                        <input
                                            type="email"
                                            class="form-input"
                                            placeholder="you@example.com"
                                            prop:value=email
                                            on:input=move |ev| set_email.set(event_target_value(&ev))
                                            required
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label class="form-label">"Message"</label>
                                        <textarea
                                            class="form-input form-textarea"
                                            placeholder="Tell us about your project..."
                                            rows="5"
                                            prop:value=message
                                            on:input=move |ev| set_message.set(event_target_value(&ev))
                                            required
                                        ></textarea>
                                    </div>
                                    <button type="submit" class="btn btn-primary btn-full">
                                        "Send Message"
                                        <span class="btn-arrow">"→"</span>
                                    </button>
                                </form>
                            }.into_any()
                        }}
                    </div>

                    <div class="contact-info">
                        <div class="info-card">
                            <div class="info-icon">"📧"</div>
                            <h4>"Email"</h4>
                            <p>"hello@stellar.dev"</p>
                        </div>
                        <div class="info-card">
                            <div class="info-icon">"📍"</div>
                            <h4>"Location"</h4>
                            <p>"San Francisco, CA"</p>
                        </div>
                        <div class="info-card">
                            <div class="info-icon">"🕐"</div>
                            <h4>"Response Time"</h4>
                            <p>"Within 24 hours"</p>
                        </div>
                        <div class="info-card info-card-social">
                            <div class="social-links">
                                <a href="#" class="social-link">"GitHub"</a>
                                <a href="#" class="social-link">"Twitter"</a>
                                <a href="#" class="social-link">"Discord"</a>
                                <a href="#" class="social-link">"LinkedIn"</a>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
