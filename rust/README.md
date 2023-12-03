## zero-ui (Rust)

Open source UI library for zero projects.

### zero-ui-leptos

install tools:

```
cargo install trunk
cargo install --locked cargo-leptos
cargo install --force cargo-make
cargo install --locked wasm-bindgen-cli
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
pnpm add -g tailwindcss
```

start command:

```
trunk serve --open
```

### zero-ui-tauri

start command

```
cargo tauri dev
```

generate icons, https://tauri.app/v1/guides/features/icons/

```
cargo tauri icon [INPUT]
<INPUT>    Path to the source icon (png, 1024x1024px with transparency) [default: ./app-icon.png]
```

### Deploy

Cloudflare Pages trunk build command

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh -s -- -y; source "$HOME/.cargo/env";rustup target add wasm32-unknown-unknown;cargo install --locked trunk;trunk build --release
```

### Setup LeptosFMT

```
cargo install leptosfmt
```

```
// .vscode/settings.json
"rust-analyzer.rustfmt.overrideCommand": ["leptosfmt", "--stdin", "--rustfmt"]
```

### Miscellaneous

clear space:

```
cargo clean
```

### References

- https://docs.rs/leptos/latest/leptos/
- https://leptos-rs.github.io/leptos/
- https://tauri.app/v1/guides/
- https://doc.rust-lang.org/cargo/
- https://doc.rust-lang.org/book/

Inspired by

- https://github.com/lpotthast/leptonic
- https://github.com/thaw-ui/thaw
