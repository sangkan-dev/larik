# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Development

Install dependencies and run the standard checks:

```bash
pnpm install
pnpm check
pnpm lint
pnpm format:check
pnpm build
```

Git hooks are managed by Husky and installed by the package `prepare` script. The default hooks run:

- `pre-commit`: `pnpm format:check`, `pnpm lint`, and `cargo fmt --check`
- `pre-push`: `pnpm build`

Rust checks run from the Tauri workspace:

```bash
cd src-tauri
cargo fmt --check
cargo check
```

On Linux, install the Tauri WebKit GTK development dependencies for your distribution before running `cargo check` or `pnpm tauri dev`.

## Tauri MCP Bridge

This app includes `tauri-plugin-mcp-bridge` for local MCP-driven Tauri inspection and automation during development.

- The Rust plugin is enabled only in debug builds.
- The bridge binds to `127.0.0.1` by default.
- The default Tauri capability grants `mcp-bridge:default`.
- The optional TypeScript bindings are installed as `@hypothesi/tauri-plugin-mcp-bridge`.

Run the app in dev mode:

```bash
pnpm tauri dev
```

Configure an MCP client with the companion server:

```bash
npx -y install-mcp @hypothesi/tauri-mcp-server --client codex
```

The bridge listens on port `9223`, or the next available port in the plugin's configured range.
