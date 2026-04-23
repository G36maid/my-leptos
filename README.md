# Stellar

A beautiful, dark-themed web application built with [Leptos](https://leptos.dev/) and Rust, compiled to WebAssembly.

**Live demo:** [g36maid.github.io/my-leptos](https://g36maid.github.io/my-leptos/)

## Tech Stack

- **[Leptos](https://leptos.dev/) 0.8** — Reactive web framework for Rust
- **[Trunk](https://trunkrs.dev/)** — Build tool for Rust WASM apps
- **WebAssembly** — Compiled via `wasm32-unknown-unknown` target
- **GitHub Actions** — CI/CD with automatic GitHub Pages deployment

## Features

- Glassmorphism navbar with backdrop blur
- Animated gradient hero section with floating orbs
- Feature cards with hover glow effects
- Filterable project showcase with status badges
- Skill bars with gradient fills
- Timeline component
- Reactive contact form with validation
- Fully responsive design (mobile hamburger menu)

## Getting Started

### Prerequisites

```bash
# Install Trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Development

```bash
trunk serve --open
```

### Production Build

```bash
trunk build --release
```

Output is written to `dist/`.

## Project Structure

```
src/main.rs              WASM entry point
src/app.rs               Router + layout shell + footer
src/components/*.rs       Page components (one per route)
style/main.css            All styles (CSS custom properties)
index.html                Trunk HTML entry
```

**Adding a new page:**
1. Create `src/components/your_page.rs`
2. Register in `src/components/mod.rs` (alphabetical order)
3. Add `<Route>` in `src/app.rs`

## CI/CD

The pipeline runs on every push/PR to `main`:

1. **Check & Lint** — `cargo fmt`, `cargo clippy`, `cargo check` (WASM target)
2. **Build** — `trunk build --release`
3. **Deploy** — Automatic GitHub Pages deployment on `main` push

## License

MIT
