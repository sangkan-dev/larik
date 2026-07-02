# AGENTS.md — Larik Editor

This file is intended for AI coding agents such as Codex, Cursor-like agents, OpenCode, or other repo automation tools.

Follow this document when making code changes in the Larik repository.

---

## 1. Project Overview

**Larik** is a lightweight, open-source, project-aware code editor.

It is not intended to be a VS Code clone and should not attempt full VS Code extension compatibility in early versions.

Larik focuses on:

- fast local editing;
- project-aware workflow;
- LSP-first language support;
- integrated terminal;
- useful run/build/deploy actions;
- optional, cost-controlled AI;
- open-source maintainability.

Primary product flow:

```txt
open project -> edit code -> run command -> inspect output -> fix issue -> prepare deploy
```

---

## 2. Product Direction

### Do

- Prioritize lightweight local workflows.
- Build features that help users code, run, inspect, fix, and deploy projects.
- Prefer standards and protocols: LSP, DAP, Tree-sitter, Git CLI, local config files.
- Keep AI optional and manual-triggered.
- Design code for readability and modularity.
- Keep the app useful without login, internet, AI, or cloud services.

### Do Not

- Do not implement full VS Code extension compatibility unless explicitly planned.
- Do not add an extension marketplace.
- Do not make AI required for core features.
- Do not auto-send workspace content to any AI provider.
- Do not auto-run terminal commands after opening a project.
- Do not introduce heavy dependencies without clear justification.
- Do not turn Larik into a generic clone of VS Code.

---

## 3. Tech Stack

Initial stack:

```txt
Desktop Shell   : Tauri v2
Backend Core    : Rust
Frontend UI     : Svelte + TypeScript
Styling         : Tailwind CSS
Editor          : Monaco Editor
Terminal UI     : xterm.js
Terminal Backend: Rust PTY backend
Syntax          : Tree-sitter
Language        : LSP client
Debug           : DAP later
Git             : git CLI first
Config          : TOML + JSON compatibility
State           : local files first, SQLite later if needed
AI              : optional BYOK/OpenAI-compatible/Ollama later
License         : Apache-2.0 OR MIT
```

---

## 4. Source Layout

Expected MVP structure:

```txt
larik/
  src/
    lib/
      components/
      panes/
      stores/
      commands/
      editor/
      terminal/
      workspace/
  src-tauri/
    src/
      main.rs
      commands/
      workspace/
      terminal/
      project_detector/
      lsp/
      git/
  docs/
    PRD.md
    TASK.md
  AGENTS.md
  README.md
  package.json
  tauri.conf.json
```

If the repository later becomes modular, use this direction:

```txt
larik/
  apps/
    desktop/
      src/
      src-tauri/
  crates/
    larik-core/
    larik-workspace/
    larik-fs/
    larik-lsp/
    larik-terminal/
    larik-project-detector/
    larik-git/
    larik-ai/
  packages/
    ui/
    themes/
    icons/
```

Do not restructure the repo heavily unless the task explicitly asks for it.

---

## 5. Implementation Priorities

When unsure, follow this order:

1. Core editor usability.
2. Workspace and file management.
3. Integrated terminal.
4. Project detection.
5. Project actions.
6. Git status.
7. LSP integration.
8. Tree-sitter-based intelligence.
9. Theme/keybinding comfort.
10. Optional AI.
11. DAP/debugging.
12. Sakala integration.

Do not start with AI, debugger, marketplace, or complex plugin systems.

---

## 6. UX Principles

Larik should feel:

- fast;
- calm;
- practical;
- minimal but not empty;
- developer-friendly;
- project-aware;
- not overloaded with extension-like noise.

Avoid UI patterns that require users to configure many things before coding.

For common code-editor workflows, prefer familiar interaction models. Larik is not a VS Code clone and must not chase full VS Code extension compatibility, but basic workflows should preserve developer muscle memory unless there is a clear product reason to differ.

Use familiar defaults for:

- Explorer: dense file tree, nested folders, immediate expand/collapse, automatic refresh after open/create/rename/delete.
- Git/source control: repository summary, staged changes, unstaged changes, untracked files, diff view, gutter indicators, explicit stage/unstage/commit actions.
- Editor tabs: active/dirty/close states.
- Command palette and quick open: keyboard-first searchable lists.
- Terminal: explicit new/run/copy/paste/kill controls, no automatic command execution.

Improve where Larik has a clear advantage: project-aware actions, safer local-first guardrails, clearer empty/error states, and simpler defaults. Do not make core interactions different just to look unique.

Default first-run experience:

```txt
Install -> Open Folder -> Start Coding
```

No login should be required for local editor features.

---

## 7. Rust Guidelines

### General

- Prefer clear modules over large files.
- Keep Tauri commands thin when possible.
- Move business logic into modules under `src-tauri/src/`.
- Return structured error types where reasonable.
- Avoid `unwrap()` in production paths.
- Use `anyhow` or thiserror-style errors if already adopted by the repo.
- Keep filesystem operations scoped to the active workspace when possible.

### Tauri Commands

Tauri commands should:

- validate input paths;
- avoid exposing arbitrary destructive operations;
- return serializable structs;
- avoid long blocking work on the UI thread;
- emit events for streaming processes such as terminal output.

Example style:

```rust
#[tauri::command]
async fn read_file(path: String) -> Result<ReadFileResponse, String> {
    // Validate path against active workspace before reading.
    // Return structured response.
    todo!()
}
```

### Safety

Never implement a command that silently:

- deletes user files;
- runs project commands automatically;
- modifies Git history;
- sends data to a network service;
- applies AI edits without confirmation.

---

## 8. Frontend Guidelines

### Svelte / TypeScript

- Use TypeScript for all frontend logic.
- Keep UI components small and focused.
- Prefer stores for app-level state:
  - workspace;
  - open tabs;
  - active file;
  - terminal sessions;
  - command registry;
  - settings.
- Keep Tauri IPC calls in dedicated service modules.
- Do not call Tauri commands directly from deeply nested components if a service/store layer is more appropriate.

### UI Structure

Recommended high-level panes:

- Explorer pane.
- Editor pane.
- Terminal pane.
- Problems pane.
- Project actions pane.
- Git pane.

### Styling

- Use Tailwind utility classes if Tailwind is configured.
- Avoid hardcoded colors in many components.
- Prefer design tokens / CSS variables for themes.
- Keep light and dark theme support in mind.

---

## 9. Editor Guidelines

Initial editor implementation uses Monaco.

Do not build a custom text editor core unless explicitly requested.

Editor responsibilities:

- open file;
- edit file;
- save file;
- track dirty state;
- map file extension to language;
- expose diagnostics markers;
- support basic keybindings;
- integrate LSP features gradually.

Future custom editor core should only be considered after the product direction is validated.

---

## 10. Terminal Guidelines

Terminal architecture:

```txt
Svelte UI -> xterm.js -> Tauri IPC/events -> Rust PTY backend -> shell process
```

Requirements:

- terminal must start in the active workspace root;
- terminal output should stream via events;
- terminal input should be sent safely;
- terminal resize should be supported;
- terminal process should be killable;
- multiple sessions should be supported later.

Do not auto-run commands without user action.

---

## 11. Project Detector Guidelines

Project detection should be local, fast, and conservative.

Initial detectors:

- Node / JS / TS via `package.json`;
- SvelteKit via dependencies or config files;
- PHP / Laravel via `composer.json` and `artisan`;
- Rust via `Cargo.toml`;
- Go via `go.mod`;
- Docker Compose via `docker-compose.yml` or `compose.yml`;
- env status via `.env` and `.env.example`.

Detector output should include:

```txt
- project type
- confidence
- detected files
- suggested actions
- warnings
```

Suggested actions must not run automatically.

---

## 12. LSP Guidelines

Larik is LSP-first.

Initial LSP manager should support:

- starting a language server process;
- JSON-RPC communication;
- initialize/shutdown;
- didOpen/didChange/didSave/didClose;
- diagnostics;
- completion;
- hover;
- go to definition;
- formatting later.

Start with one language server and stabilize the manager before adding many languages.

Avoid hardcoding too much language-specific behavior into the core. Use config/adapters where practical.

---

## 13. Tree-sitter Guidelines

Tree-sitter is used for project intelligence and syntax-aware features, not as a blocker for MVP editing.

Initial use cases:

- parse active file;
- extract symbols;
- build simple outline;
- support lightweight context extraction.

Do not delay MVP file editing for full Tree-sitter integration.

---

## 14. Git Guidelines

Initial Git integration should use the Git CLI for predictable behavior.

Allowed initial features:

- detect Git repo;
- read active branch;
- read changed files;
- show status in UI.

Avoid complex Git operations early:

- rebase;
- reset hard;
- force push;
- history rewrite.

Destructive Git actions must require explicit confirmation.

---

## 15. AI Guidelines

AI is optional and should not be implemented before the core local editor workflow is stable.

Rules:

- AI must be off by default.
- AI must require user action.
- AI must support BYOK or custom endpoint design.
- AI must not auto-send full workspace context.
- AI should default to selected text, selected diff, or selected terminal output.
- AI edits must be shown as diff before applying.
- AI must not run terminal commands automatically.

Good initial AI commands:

- Explain selected code.
- Explain selected terminal error.
- Generate commit message from selected diff.
- Review selected diff.
- Suggest Dockerfile.
- Suggest `.env.example`.

Avoid building autonomous agents in early versions.

---

## 16. Sakala Integration Guidelines

Sakala integration is a future optional feature.

Larik must remain useful without Sakala account.

Future Sakala features may include:

- detect deploy readiness;
- preview build plan;
- deploy project;
- view deployment logs;
- open public URL;
- rollback.

Do not tightly couple Larik core to Sakala APIs. Use an optional module/integration layer.

---

## 17. Testing Guidelines

When adding code, also consider tests for:

- project detector;
- path normalization;
- file operations;
- settings parsing;
- command registry;
- LSP message parsing;
- Git status parsing.

Prefer unit tests for Rust logic that does not require Tauri runtime.

For frontend:

- keep logic in testable stores/services;
- avoid putting complex logic only inside components.

---

## 18. Documentation Rules

When changing scope or behavior:

- update `PRD.md` if product direction changes;
- update `TASK.md` if tasks are added/completed/reprioritized;
- update `AGENTS.md` if development conventions change;
- update `README.md` for user-facing setup or usage changes.
- update `DESIGN_SYSTEM.md` if design tokens or theme variables change.

Do not let implementation drift far from the docs.

---

## 19. Development Commands

These commands are placeholders until the repository is initialized.

Expected commands:

```bash
pnpm install
pnpm tauri dev
pnpm check
pnpm lint
pnpm format
cargo fmt
cargo clippy
cargo test
```

If the actual package manager is different, update this section.

---

## 20. Acceptance Checklist for Agent Changes

Before finishing any task, verify:

- [ ] The change matches the current PRD direction.
- [ ] The change updates TASK.md if task status changed.
- [ ] No unnecessary heavy dependency was added.
- [ ] No AI/network behavior was added without explicit user action.
- [ ] No destructive file/terminal/Git operation runs automatically.
- [ ] Rust code avoids unsafe `unwrap()` in user-facing paths.
- [ ] Frontend code keeps IPC and business logic out of deeply nested components where practical.
- [ ] The app can still run local-first.
- [ ] Documentation is updated if behavior changed.

---

## 21. Current Recommended First Steps

If starting from an empty repository, do this order:

1. Bootstrap Tauri v2 + Svelte + TypeScript.
2. Add Tailwind CSS.
3. Create base layout: sidebar, editor area, bottom panel, status bar.
4. Implement Open Folder.
5. Implement File Explorer.
6. Integrate Monaco Editor.
7. Implement open/edit/save file.
8. Implement tabs.
9. Integrate xterm.js and Rust PTY backend.
10. Implement project detector root scan.
11. Implement project actions panel.
12. Add basic Git status.
13. Add LSP skeleton.

Stop before adding AI or debugger unless explicitly requested.
