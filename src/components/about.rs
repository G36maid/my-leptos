use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    static SKILLS: &[(&str, i32)] = &[
        ("Rust", 95),
        ("WebAssembly", 90),
        ("Leptos", 88),
        ("TypeScript", 75),
        ("CSS/Tailwind", 85),
        ("Systems Design", 80),
    ];

    view! {
        <div class="page page-about">
            <section class="section about-hero">
                <span class="section-tag">"About Us"</span>
                <h1 class="about-title">
                    "Passionate about "
                    <span class="text-gradient">"Rust"</span>
                    " & the web"
                </h1>
                <p class="about-intro">
                    "We believe in building software that is fast, reliable, and maintainable.
                    Rust and WebAssembly are transforming the web — and we're at the forefront."
                </p>
            </section>

            <section class="section mission-section">
                <div class="mission-grid">
                    <div class="mission-card mission-card-primary">
                        <div class="mission-number">"01"</div>
                        <h3>"Our Mission"</h3>
                        <p>
                            "To demonstrate that web applications can be both beautiful and performant,
                            without compromising on developer experience or type safety."
                        </p>
                    </div>
                    <div class="mission-card">
                        <div class="mission-number">"02"</div>
                        <h3>"Our Stack"</h3>
                        <p>
                            "Built entirely with Rust — from the backend APIs to the frontend framework.
                            Leptos provides the reactive primitives, WebAssembly delivers the speed."
                        </p>
                    </div>
                    <div class="mission-card">
                        <div class="mission-number">"03"</div>
                        <h3>"Our Vision"</h3>
                        <p>
                            "A future where web developers don't have to choose between safety and productivity.
                            Where compile-time checks eliminate entire classes of bugs."
                        </p>
                    </div>
                </div>
            </section>

            <section class="section skills-section">
                <div class="section-header">
                    <span class="section-tag">"Expertise"</span>
                    <h2 class="section-title">"Skills & Technologies"</h2>
                </div>
                <div class="skills-list">
                    {SKILLS.iter().map(|(name, level)| {
                        view! {
                            <div class="skill-item">
                                <div class="skill-header">
                                    <span class="skill-name">{*name}</span>
                                    <span class="skill-percent">{*level}"%"</span>
                                </div>
                                <div class="skill-bar">
                                    <div
                                        class="skill-fill"
                                        style=move || format!("width: {}%", level)
                                    ></div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </section>

            <section class="section timeline-section">
                <div class="section-header">
                    <span class="section-tag">"Journey"</span>
                    <h2 class="section-title">"Our Story"</h2>
                </div>
                <div class="timeline">
                    <TimelineItem year="2024" title="The Beginning" description="Discovered the power of Rust for web development and started experimenting with WebAssembly."/>
                    <TimelineItem year="2025" title="Leptos Adoption" description="Migrated to Leptos framework for its elegant reactive primitives and full-stack capabilities."/>
                    <TimelineItem year="2026" title="Stellar Launch" description="Launched Stellar — a showcase of what's possible with Rust on the modern web."/>
                </div>
            </section>
        </div>
    }
}

#[component]
fn TimelineItem(
    year: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="timeline-item">
            <div class="timeline-marker">
                <div class="timeline-dot"></div>
            </div>
            <div class="timeline-content">
                <span class="timeline-year">{year}</span>
                <h3 class="timeline-title">{title}</h3>
                <p class="timeline-desc">{description}</p>
            </div>
        </div>
    }
}
