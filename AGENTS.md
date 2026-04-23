# AGENTS.md

## Build & Run

- **Trunk** is the build tool (not `cargo-leptos`). Dev: `trunk serve --open`. Prod: `trunk build --release`.
- WASM target `wasm32-unknown-unknown` is required: `rustup target add wasm32-unknown-unknown`.
- `cargo check` alone won't catch WASM issues — run `cargo check --target wasm32-unknown-unknown`.

## CI Pipeline

`cargo fmt` → `cargo clippy -D warnings` → `cargo check --target wasm32-unknown-unknown` → `trunk build --release` → GitHub Pages deploy.

All must pass before merge. Run locally before pushing:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --target wasm32-unknown-unknown
trunk build --release
```

## Leptos 0.8 Quirks

- **Edition 2024** (`Cargo.toml`).
- `leptos` uses `features = ["csr"]`; `leptos_router` has no CSR feature — don't add one.
- `<A>` from `leptos_router` supports `class` via `attr:class="..."` and `on:click` normally. Use `<A>` for all router-aware navigation links. Plain `<a>` bypasses the router base path.
- The `view!` macro rejects certain Unicode characters (e.g. `✦`) outside string literals. Keep special chars inside `"..."` strings only.
- Static data in components (e.g. skill arrays) must be `static` bindings, not `let`, or the view closure borrow checker will reject them.
- `cargo fmt` enforces alphabetical `mod`/`pub use` ordering in `mod.rs` files.

## Architecture

```
src/main.rs          — WASM entrypoint (mount_to_body)
src/app.rs           — Router + layout shell + footer
src/components/*.rs  — Page components (one file per route)
style/main.css       — All styles (Trunk injects via <link data-trunk rel="css">)
index.html           — Trunk HTML entry (not public/index.html)
```

All routes are defined in `app.rs`. New pages: create component file → add `mod`/`pub use` in `components/mod.rs` → add `<Route>` in `app.rs`.

## Conventions

- Conventional commits: `feat:`, `fix:`, `ci:`, `chore:`, `style:`.
- Dark theme CSS with CSS custom properties. All styles in single `style/main.css`.
- No tests yet.
