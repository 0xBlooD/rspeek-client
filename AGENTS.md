# AGENTS.md — rspeek-client

> Read the global rules first: `../.agents/rules/` (umbrella folder). If you cannot access them, the essentials are repeated here.

## What this repo is
The **client suite** for RSpeek:
- `crates/rspeek-client-core` — state machine and networking (native + wasm32, NO UI)
- `crates/rspeek-ui` — egui presentation components (desktop + wasm32, NO networking)
- `apps/rspeek` — Desktop GUI client (`eframe` native) — **PRIMARY TARGET**
- `apps/rspeek-web` — Web WASM client (`eframe` + `trunk`)
- `apps/rspeek-cli` — Terminal client (`ratatui` + `crossterm`, NO egui)

## Hard rules
1. **When in doubt, ask the owner. Never guess.**
2. **Strict separation of concerns:**
   - `client-core` has NO UI code and compiles to `wasm32-unknown-unknown`.
   - `rspeek-ui` has NO networking code and compiles to `wasm32-unknown-unknown`.
   - `rspeek-cli` does NOT depend on egui or `rspeek-ui`.
3. Desktop client (`apps/rspeek`) is the primary active focus. Mobile is far off in the future.
4. `rspeek-shared` is consumed via git tag: `rspeek-shared = { git = "ssh://git@github.com/0xBlooD/rspeek-shared.git", tag = "v0.1.0" }`. Local development uses gitignored `[patch]`.
5. Keep it simple and debuggable (rule `05-simplicity.md`).
6. Everything in English. Conventional Commits. git-flow.

## Verify before "done"
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --target wasm32-unknown-unknown -p rspeek-client-core -p rspeek-ui -p rspeek-web
```
