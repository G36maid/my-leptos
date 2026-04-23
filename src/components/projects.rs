use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    let projects = vec![
        ProjectData {
            title: "Stellar Dashboard",
            description: "A real-time analytics dashboard with interactive charts, live data streaming, and dark/light theme support.",
            tags: vec!["Leptos", "Chart.js", "WASM"],
            color: "purple",
            status: "Live",
        },
        ProjectData {
            title: "RustChat",
            description: "End-to-end encrypted messaging app with real-time WebSocket communication and file sharing.",
            tags: vec!["Rust", "WebSocket", "Crypto"],
            color: "blue",
            status: "Beta",
        },
        ProjectData {
            title: "CodeForge",
            description: "Online collaborative code editor with syntax highlighting, auto-completion, and integrated terminal.",
            tags: vec!["Leptos", "Monaco", "WASM"],
            color: "emerald",
            status: "Alpha",
        },
        ProjectData {
            title: "PixelCraft",
            description: "Browser-based image editor with GPU-accelerated filters, layer support, and WebAssembly processing.",
            tags: vec!["Rust", "WebGPU", "WASM"],
            color: "orange",
            status: "Live",
        },
        ProjectData {
            title: "TaskFlow",
            description: "Project management tool with Kanban boards, Gantt charts, and team collaboration features.",
            tags: vec!["Leptos", "SSR", "SQLite"],
            color: "pink",
            status: "Development",
        },
        ProjectData {
            title: "NeuralViz",
            description: "Interactive neural network visualizer that demonstrates forward passes, backpropagation, and training.",
            tags: vec!["Rust", "Canvas", "WASM"],
            color: "cyan",
            status: "Live",
        },
    ];

    let (filter, set_filter) = signal("all");
    let filtered_projects = move || {
        let current = filter.get();
        if current == "all" {
            projects.clone()
        } else {
            projects
                .iter()
                .filter(|p| p.status.to_lowercase() == current)
                .cloned()
                .collect::<Vec<_>>()
        }
    };

    view! {
        <div class="page page-projects">
            <section class="section projects-hero">
                <span class="section-tag">"Portfolio"</span>
                <h1 class="projects-title">
                    "Our "
                    <span class="text-gradient">"Projects"</span>
                </h1>
                <p class="projects-intro">
                    "A collection of applications showcasing the power of Rust and Leptos on the web."
                </p>

                // Filter buttons
                <div class="filter-bar">
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == "all"
                        on:click=move |_| set_filter.set("all")
                    >
                        "All"
                    </button>
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == "live"
                        on:click=move |_| set_filter.set("live")
                    >
                        "Live"
                    </button>
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == "beta"
                        on:click=move |_| set_filter.set("beta")
                    >
                        "Beta"
                    </button>
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == "alpha"
                        on:click=move |_| set_filter.set("alpha")
                    >
                        "Alpha"
                    </button>
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == "development"
                        on:click=move |_| set_filter.set("development")
                    >
                        "In Dev"
                    </button>
                </div>
            </section>

            <section class="section projects-grid-section">
                <div class="projects-grid">
                    {move || {
                        filtered_projects()
                            .into_iter()
                            .map(|project| {
                                view! {
                                    <ProjectCard project=project/>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </section>
        </div>
    }
}

#[derive(Clone)]
struct ProjectData {
    title: &'static str,
    description: &'static str,
    tags: Vec<&'static str>,
    color: &'static str,
    status: &'static str,
}

#[component]
fn ProjectCard(project: ProjectData) -> impl IntoView {
    let status_class = match project.status {
        "Live" => "status-live",
        "Beta" => "status-beta",
        "Alpha" => "status-alpha",
        _ => "status-dev",
    };

    view! {
        <div class=format!("project-card project-{}", project.color)>
            <div class="project-header">
                <div class="project-status">
                    <span class=format!("status-badge {}", status_class)>
                        {project.status}
                    </span>
                </div>
                <div class="project-dots">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            </div>
            <h3 class="project-title">{project.title}</h3>
            <p class="project-desc">{project.description}</p>
            <div class="project-tags">
                {project.tags.iter().map(|tag| {
                    view! {
                        <span class="project-tag">{*tag}</span>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div class="project-footer">
                <span class="project-link">
                    "View Details →"
                </span>
            </div>
        </div>
    }
}
