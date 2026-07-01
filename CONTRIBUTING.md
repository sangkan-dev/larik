# Contributing to Larik

Larik is a lightweight, project-aware code editor. Contributions should keep the app local-first, fast, and useful without AI, login, or cloud services.

## Development Setup

```bash
pnpm install
pnpm check
pnpm build
cd src-tauri && cargo fmt --check
```

For desktop builds on Linux, install the Tauri system dependencies for your distribution first.

## Working Guidelines

- Follow `PRD.md`, `DESIGN_SYSTEM.md`, `TASK.md`, and `AGENTS.md`.
- Prefer small, focused changes.
- Keep core workflows local and explicit.
- Do not add heavy dependencies without clear product value.
- Do not auto-run project commands or send workspace data to external services.
- Update `TASK.md` when completing or changing planned work.

## License

Unless stated otherwise, contributions are accepted under `Apache-2.0 OR MIT`.
