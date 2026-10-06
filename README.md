# rspeek-client

Client suite for [RSpeek](https://github.com/0xBlooD): desktop GUI, web browser WASM, and terminal TUI.

## Structure
- `crates/rspeek-client-core`: Shared connection handling and state management. Compiles to native and WASM.
- `crates/rspeek-ui`: Shared egui presentation widgets and layouts.
- `apps/rspeek`: **Desktop app** (egui/eframe native) — primary target.
- `apps/rspeek-web`: **Web app** (egui/eframe compiled to WASM via Trunk).
- `apps/rspeek-cli`: **Terminal client** (ratatui + crossterm).

## Running Desktop App
```bash
cargo run -p rspeek
```

## Running Web App (Trunk)
Requires [trunk](https://trunkrs.dev/):
```bash
cd apps/rspeek-web
trunk serve
```

## Running Terminal TUI
```bash
cargo run -p rspeek-cli
```

## Testing & Linting
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --target wasm32-unknown-unknown -p rspeek-client-core -p rspeek-ui -p rspeek-web
```

## License
Proprietary. All rights reserved. See [LICENSE](LICENSE).
