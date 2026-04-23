use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="page page-home">
            <section class="hero">
                <div class="hero-bg">
                    <div class="hero-orb hero-orb-1"></div>
                    <div class="hero-orb hero-orb-2"></div>
                    <div class="hero-orb hero-orb-3"></div>
                </div>
                <div class="hero-content">
                    <div class="hero-badge">
                        <span class="badge-dot"></span>
                        "Powered by Leptos & WebAssembly"
                    </div>
                    <h1 class="hero-title">
                        <span class="title-line">"Welcome to"</span>
                        <span class="title-gradient">"Stellar"</span>
                    </h1>
                    <div class="hero-typing">
                        <span class="typing-text">"Building the future with Rust"</span>
                        <span class="typing-cursor">"|"</span>
                    </div>
                    <p class="hero-description">
                        "A blazingly fast, beautiful web application built entirely with Rust.
                        Experience the power of reactive programming with type safety."
                    </p>
                    <div class="hero-actions">
                        <a href="/projects" class="btn btn-primary">
                            "Explore Projects"
                            <span class="btn-arrow">"→"</span>
                        </a>
                        <a href="/about" class="btn btn-ghost">
                            "Learn More"
                        </a>
                    </div>
                    <div class="hero-stats">
                        <div class="stat">
                            <span class="stat-value">"100%"</span>
                            <span class="stat-label">"Rust"</span>
                        </div>
                        <div class="stat-divider"></div>
                        <div class="stat">
                            <span class="stat-value">"<50ms"</span>
                            <span class="stat-label">"First Paint"</span>
                        </div>
                        <div class="stat-divider"></div>
                        <div class="stat">
                            <span class="stat-value">"0"</span>
                            <span class="stat-label">"Runtime Errors"</span>
                        </div>
                    </div>
                </div>
            </section>

            <section class="section features">
                <div class="section-header">
                    <span class="section-tag">"Why Leptos?"</span>
                    <h2 class="section-title">"Built for Performance"</h2>
                    <p class="section-subtitle">"Leverage the full power of Rust on the web"</p>
                </div>
                <div class="features-grid">
                    <FeatureCard
                        icon="⚡"
                        title="Blazing Fast"
                        description="Compiled to WebAssembly for near-native performance. No virtual DOM overhead."
                    />
                    <FeatureCard
                        icon="🔒"
                        title="Type Safe"
                        description="Catch errors at compile time, not runtime. Rust's type system has your back."
                    />
                    <FeatureCard
                        icon="🧩"
                        title="Reactive"
                        description="Fine-grained reactivity without a virtual DOM. Only update what changes."
                    />
                    <FeatureCard
                        icon="📦"
                        title="Tiny Bundle"
                        description="WebAssembly produces incredibly small bundles. Fast loading guaranteed."
                    />
                    <FeatureCard
                        icon="🎨"
                        title="Modern UI"
                        description="Beautiful interfaces with the view macro. Declarative and expressive."
                    />
                    <FeatureCard
                        icon="🔄"
                        title="Isomorphic"
                        description="Same code runs on client and server. SSR, CSR, or hydration — your choice."
                    />
                </div>
            </section>

            <section class="section cta-section">
                <div class="cta-card">
                    <div class="cta-glow"></div>
                    <h2>"Ready to build something amazing?"</h2>
                    <p>"Join the Rust web revolution and create performant applications with confidence."</p>
                    <a href="/contact" class="btn btn-primary btn-lg">
                        "Get Started"
                        <span class="btn-arrow">"→"</span>
                    </a>
                </div>
            </section>
        </div>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3 class="feature-title">{title}</h3>
            <p class="feature-desc">{description}</p>
        </div>
    }
}
