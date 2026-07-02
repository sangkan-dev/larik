# TASK — Larik Editor

> **Status:** Draft awal 0.1  
> **Produk:** Larik Editor  
> **Tujuan:** Menjadi task backlog awal untuk membangun MVP Larik secara bertahap.

---

## Legend

- `[ ]` belum dikerjakan
- `[~]` sedang dikerjakan
- `[x]` selesai
- `[!]` blocked / perlu keputusan

---

## 0. Project Setup

### 0.1 Repository

- [x] Buat repo `sangkan-dev/larik`.
- [x] Tambahkan `README.md` awal.
- [x] Tambahkan `PRD.md`.
- [x] Tambahkan `TASK.md`.
- [x] Tambahkan `AGENTS.md`.
- [x] Tambahkan license `Apache-2.0 OR MIT`.
- [x] Tambahkan `.gitignore` untuk Rust, Node, Tauri, OS files.
- [x] Tambahkan `CONTRIBUTING.md` sederhana.
- [x] Tambahkan `CODE_OF_CONDUCT.md` jika repo mulai dibuka untuk publik.

### 0.2 Tooling

- [x] Setup Tauri v2 project.
- [x] Setup Svelte + TypeScript.
- [x] Setup Tailwind CSS.
- [x] Setup formatter frontend.
- [x] Setup formatter Rust.
- [x] Setup linting frontend.
- [x] Setup Git hooks via Husky.
- [x] Setup GitHub Actions basic CI.
- [x] Setup build check untuk Linux.

### 0.3 Project Metadata

- [x] Tentukan app name: `Larik`.
- [x] Tentukan binary name: `larik`.
- [x] Tentukan identifier aplikasi Tauri.
- [x] Tambahkan app icon sementara.
- [x] Tambahkan dark/light theme sementara.

---

## 1. Desktop Shell MVP

### 1.1 Tauri Window

- [x] App bisa booting.
- [x] Window title memakai `Larik`.
- [x] Default window size nyaman untuk coding.
- [x] Persist window size dan position.
- [x] Tambahkan menu dasar.
- [x] Tambahkan shortcut quit/reload/devtools untuk dev build.

### 1.2 Layout Dasar

- [x] Buat layout utama:
  - [x] activity bar / left rail minimal;
  - [x] sidebar explorer;
  - [x] editor area;
  - [x] bottom panel;
  - [x] status bar.
- [x] Tambahkan resizable panels.
- [x] Tambahkan collapse/expand sidebar.
- [x] Tambahkan collapse/expand bottom panel.

### 1.3 State UI

- [x] Buat store workspace aktif.
- [x] Buat store tabs terbuka.
- [x] Buat store active file.
- [x] Buat store panel visibility.
- [x] Simpan UI state lokal.

---

## 2. Workspace & File System

### 2.1 Open Folder

- [x] Tambahkan command `Open Folder`.
- [x] Buka native folder picker via Tauri.
- [x] Simpan folder sebagai workspace aktif.
- [x] Simpan daftar recent workspaces.
- [x] Render nama folder di status bar/window title.

### 2.2 File Explorer

- [x] Load file tree dari workspace.
- [x] Ignore folder berat:
  - [x] `.git`
  - [x] `node_modules`
  - [x] `vendor`
  - [x] `target`
  - [x] `dist`
  - [x] `build`
- [x] Expand/collapse folder.
- [x] Open file dari explorer.
- [x] Refresh explorer.
- [x] Buat file baru.
- [x] Buat folder baru.
- [x] Rename file/folder.
- [x] Delete file/folder dengan konfirmasi.

### 2.3 File Editing

- [x] Read file content via Rust command.
- [x] Tampilkan file di editor.
- [x] Save file.
- [x] Track dirty state.
- [x] Prevent close tab jika file belum disimpan.
- [x] Support binary/large file guard.
- [x] Tampilkan pesan untuk file yang terlalu besar.

### 2.4 File Watcher

- [x] Watch perubahan file workspace.
- [x] Update explorer ketika file berubah.
- [x] Deteksi file yang berubah di disk saat sedang dibuka.
- [x] Tampilkan pilihan reload/keep local.

---

## 3. Editor Core via Monaco

### 3.1 Monaco Integration

- [x] Install Monaco Editor.
- [x] Render Monaco di editor area.
- [x] Support multiple file models.
- [x] Dispose model ketika tab ditutup.
- [x] Set language mode berdasarkan file extension.
- [x] Set theme dark/light.

### 3.2 Tabs

- [x] Open file sebagai tab.
- [x] Switch tab.
- [x] Close tab.
- [x] Close other tabs.
- [x] Close all tabs.
- [x] Reopen recently closed tab.
- [x] Show dirty indicator.

### 3.3 Editor UX

- [x] Basic find in file.
- [x] Basic replace in file.
- [x] Go to line.
- [x] Minimap toggle.
- [x] Word wrap toggle.
- [x] Format document placeholder.
- [x] Save shortcut.
- [x] Save all shortcut.

### 3.4 Keybindings

- [x] Implement keybinding registry awal.
- [x] Tambahkan shortcut dasar:
  - [x] Open folder
  - [x] Save
  - [x] Save all
  - [x] Quick open
  - [x] Command palette
  - [x] Toggle terminal
  - [x] Toggle sidebar
  - [x] Close tab
- [x] Buat format keybindings JSON sederhana.

---

## 4. Command Palette & Quick Open

### 4.1 Command Registry

- [x] Buat command registry di frontend.
- [x] Buat command metadata:
  - [x] id
  - [x] title
  - [x] category
  - [x] shortcut optional
  - [x] handler
- [x] Register command dasar editor.

### 4.2 Command Palette

- [x] UI command palette.
- [x] Fuzzy search command.
- [x] Keyboard navigation.
- [x] Execute selected command.
- [x] Close on escape.

### 4.3 Quick Open File

- [x] Index file path workspace.
- [x] Fuzzy search file.
- [x] Ignore folder berat.
- [x] Open selected file.
- [x] Keyboard navigation.

---

## 5. Integrated Terminal

### 5.1 Terminal UI

- [x] Install xterm.js.
- [x] Render terminal di bottom panel.
- [x] Fit terminal size.
- [x] Support terminal resize.
- [x] Support terminal focus/blur.

### 5.2 Rust PTY Backend

- [x] Implement spawn shell.
- [x] Stream stdout/stderr ke frontend.
- [x] Kirim keyboard input ke process.
- [x] Resize PTY.
- [x] Kill terminal process.
- [x] Support multiple terminal sessions.

### 5.3 Terminal Commands

- [x] Run command di workspace root.
- [x] Buat terminal dari project action.
- [x] Tampilkan command label.
- [x] Auto-scroll output.
- [x] Basic copy/paste.

---

## 6. Project Detector

### 6.1 Detector Core

- [x] Buat module `project_detector` di Rust.
- [x] Scan root workspace secara ringan.
- [x] Return detected project types.
- [x] Return confidence score sederhana.
- [x] Return suggested actions.

### 6.2 Node / JS / TS Detector

- [x] Deteksi `package.json`.
- [x] Parse package manager:
  - [x] npm
  - [x] pnpm
  - [x] yarn
  - [x] bun
- [x] Parse scripts dari `package.json`.
- [x] Deteksi framework:
  - [x] SvelteKit
  - [x] Vite
  - [x] Next.js
  - [x] Express / generic Node
- [x] Tampilkan action `install`, `dev`, `build`, `test` jika ada.

### 6.3 PHP / Laravel Detector

- [x] Deteksi `composer.json`.
- [x] Deteksi Laravel via `artisan`.
- [x] Deteksi `app/`, `routes/`, `database/migrations`.
- [x] Tampilkan action:
  - [x] `composer install`
  - [x] `php artisan serve`
  - [x] `php artisan migrate`
  - [x] `php artisan queue:work`
  - [x] `php artisan test`
- [x] Deteksi `.env.example` dan `.env`.

### 6.4 Rust Detector

- [x] Deteksi `Cargo.toml`.
- [x] Parse package name.
- [x] Tampilkan action:
  - [x] `cargo check`
  - [x] `cargo run`
  - [x] `cargo test`
  - [x] `cargo build`

### 6.5 Go Detector

- [x] Deteksi `go.mod`.
- [x] Parse module name.
- [x] Tampilkan action:
  - [x] `go run .`
  - [x] `go test ./...`
  - [x] `go build ./...`

### 6.6 Docker Compose Detector

- [x] Deteksi `docker-compose.yml`.
- [x] Deteksi `compose.yml`.
- [x] Tampilkan action:
  - [x] `docker compose up`
  - [x] `docker compose up -d`
  - [x] `docker compose down`
  - [x] `docker compose logs -f`
  - [x] `docker compose ps`

### 6.7 Env Detector

- [x] Deteksi `.env`.
- [x] Deteksi `.env.example`.
- [x] Compare missing keys.
- [x] Tampilkan warning jika `.env` belum ada.
- [x] Tampilkan daftar key yang belum terisi.

---

## 7. Project Actions Panel

### 7.1 UI

- [x] Buat panel `Project`.
- [x] Tampilkan detected project type.
- [x] Tampilkan suggested actions.
- [x] Tampilkan env status.
- [x] Tampilkan scripts.

### 7.2 Action Runner

- [x] Run action di integrated terminal.
- [x] Support action working directory.
- [x] Support action label.
- [x] Tampilkan status running/done/failed.
- [x] Prevent duplicate dangerous action jika perlu.

### 7.3 Safety

- [x] Konfirmasi untuk action destructive:
  - [x] delete file;
  - [x] docker compose down;
  - [x] migration destructive di masa depan.
- [x] Jangan auto-run command setelah project dibuka.

---

## 8. Git Integration v0

### 8.1 Git Detection

- [x] Deteksi apakah workspace adalah Git repo.
- [x] Ambil branch aktif.
- [x] Ambil status file berubah.
- [x] Tampilkan indikator di status bar.

### 8.2 Git Panel

- [x] Tampilkan changed files.
- [x] Tampilkan staged/unstaged basic.
- [x] Open changed file.
- [x] Basic diff view placeholder.

### 8.3 Future Git Tasks

- [x] Stage file.
- [x] Unstage file.
- [x] Commit.
- [x] Generate commit message dengan AI optional.
- [x] View diff inline.
- [x] Git gutter.

---

## 9. LSP Integration

### 9.1 LSP Manager Skeleton

- [x] Buat module `lsp` di Rust.
- [x] Define language server config.
- [x] Start language server process.
- [x] Stop language server process.
- [x] Send initialize request.
- [x] Send shutdown request.
- [x] Handle JSON-RPC messages.

### 9.2 Document Sync

- [x] Send `textDocument/didOpen`.
- [x] Send `textDocument/didChange`.
- [x] Send `textDocument/didSave`.
- [x] Send `textDocument/didClose`.

### 9.3 Diagnostics

- [x] Receive diagnostics.
- [x] Map diagnostics to file path.
- [x] Show diagnostics in Monaco markers.
- [x] Create Problems panel.

### 9.4 Completion

- [x] Request completion from LSP.
- [x] Map completion items to Monaco completion provider.
- [ ] Support resolve completion optional.

### 9.5 Hover & Navigation

- [x] Hover provider.
- [x] Go to definition.
- [ ] Find references.
- [ ] Rename symbol.
- [x] Document formatting.
- [ ] Code action.

### 9.6 Initial Language Targets

- [x] TypeScript / JavaScript.
- [ ] Svelte.
- [ ] PHP.
- [ ] Rust.
- [ ] Go.

> Catatan: mulai dari satu bahasa dulu agar LSP manager stabil sebelum menambah bahasa lain.

---

## 10. Tree-sitter Layer

### 10.1 Setup

- [ ] Tambahkan dependency Tree-sitter di Rust.
- [ ] Tambahkan grammar awal:
  - [ ] JavaScript
  - [ ] TypeScript
  - [ ] PHP
  - [ ] Rust
  - [ ] Go
  - [ ] Markdown
- [ ] Buat parser manager.

### 10.2 Use Cases

- [ ] Parse active file.
- [ ] Generate simple symbol outline.
- [ ] Detect functions/classes.
- [ ] Support project-aware indexing ringan.
- [ ] Support context extraction untuk AI optional.

### 10.3 Later

- [ ] Syntax-aware selection.
- [ ] Folding improvement.
- [ ] Semantic search.
- [ ] Structural search.

---

## 11. Settings & Configuration

### 11.1 Settings Storage

- [ ] Tentukan config dir:
  - [ ] Linux: `~/.config/larik/`
  - [ ] macOS: sesuai app config dir
  - [ ] Windows: sesuai app data dir
- [ ] Buat `settings.toml`.
- [ ] Buat `keybindings.json`.
- [ ] Buat `workspaces.json`.

### 11.2 Settings UI

- [ ] Settings file open command.
- [ ] Basic Settings UI optional.
- [ ] Reload settings tanpa restart.

### 11.3 Supported Settings v0

- [ ] Theme.
- [ ] Font family.
- [ ] Font size.
- [ ] Tab size.
- [ ] Word wrap.
- [ ] Minimap.
- [ ] Format on save.
- [ ] Terminal shell path.

---

## 12. Theme & Icon

### 12.1 Built-in Theme

- [ ] Larik Dark.
- [ ] Larik Light.
- [ ] Basic color tokens.
- [ ] Editor theme mapping to Monaco.

### 12.2 Theme Import Later

- [ ] VS Code theme JSON subset importer.
- [ ] Theme preview.
- [ ] Theme switching.

### 12.3 Icon

- [ ] Built-in file icons sederhana.
- [ ] Extension-based icon mapping.
- [ ] Folder icon mapping.
- [ ] VS Code icon theme subset importer later.

---

## 13. AI Optional Layer

> Jangan dikerjakan sebelum editor, terminal, project detector, dan LSP dasar stabil.

### 13.1 Provider Config

- [ ] OpenAI-compatible provider.
- [ ] Custom base URL.
- [ ] API key storage lokal.
- [ ] Model name config.
- [ ] Ollama provider later.

### 13.2 AI Commands

- [ ] Explain selected code.
- [ ] Explain selected terminal error.
- [ ] Generate commit message.
- [ ] Review selected diff.
- [ ] Generate Dockerfile suggestion.
- [ ] Generate `.env.example` suggestion.

### 13.3 Cost Control

- [ ] AI off by default.
- [ ] Manual trigger only.
- [ ] Show approximate context size before send.
- [ ] Send selected text only by default.
- [ ] Do not auto-index full workspace to AI.
- [ ] Add BYOK warning.

### 13.4 Safety

- [ ] AI cannot write file without user confirmation.
- [ ] AI cannot run terminal command automatically.
- [ ] Show diff before applying AI edit.

---

## 14. Sakala Integration Later

### 14.1 Detection

- [ ] Detect deploy readiness.
- [ ] Detect required env.
- [ ] Detect build command.
- [ ] Detect runtime.

### 14.2 Actions

- [ ] Login to Sakala.
- [ ] Link project to Sakala project.
- [ ] Preview build/deploy plan.
- [ ] Deploy to Sakala.
- [ ] Show deployment logs.
- [ ] Open public URL.
- [ ] Rollback.

### 14.3 Boundaries

- [ ] Sakala integration must be optional.
- [ ] Larik must remain useful without Sakala account.
- [ ] No login required for local editor features.

---

## 15. Packaging & Release

### 15.1 Development Build

- [ ] `pnpm install` or chosen package manager.
- [ ] `cargo check`.
- [ ] `tauri dev`.
- [ ] Document dev setup.

### 15.2 CI

- [ ] Format check frontend.
- [ ] Typecheck frontend.
- [ ] Rust fmt check.
- [ ] Rust clippy.
- [ ] Rust test.
- [ ] Tauri build smoke test.

### 15.3 Release

- [ ] Build Linux AppImage/deb/rpm.
- [ ] Build Windows installer later.
- [ ] Build macOS dmg later.
- [ ] Generate changelog.
- [ ] GitHub release workflow.

---

## 16. Documentation

### 16.1 User Docs

- [ ] Getting started.
- [ ] Open folder.
- [ ] Project detection.
- [ ] Terminal.
- [ ] Settings.
- [ ] LSP setup.

### 16.2 Developer Docs

- [ ] Architecture overview.
- [ ] Frontend structure.
- [ ] Rust command structure.
- [ ] IPC contract.
- [ ] Project detector guide.
- [ ] LSP manager guide.
- [ ] Contribution guide.

### 16.3 Agent Docs

- [ ] Keep `AGENTS.md` updated.
- [ ] Add coding conventions.
- [ ] Add testing commands.
- [ ] Add scope boundaries.

---

## 17. MVP Milestones

### Milestone 0.1 — Usable Local Editor

- [ ] Tauri app booting.
- [ ] Open folder.
- [ ] File explorer.
- [ ] Open/edit/save file.
- [ ] Monaco editor.
- [ ] Multi-tab basic.
- [ ] Integrated terminal basic.
- [ ] Project detector root files.
- [ ] Basic project actions.
- [ ] Git branch and changed files.

### Milestone 0.2 — Language-aware Editor

- [x] LSP manager basic.
- [x] Diagnostics.
- [x] Completion.
- [x] Hover.
- [x] Go to definition.
- [ ] Format on save.
- [x] Problems panel.

### Milestone 0.3 — Project-aware Workflow

- [ ] Laravel preset.
- [ ] SvelteKit preset.
- [ ] Node/Vite preset.
- [ ] Rust preset.
- [ ] Go preset.
- [ ] Docker Compose preset.
- [ ] Env checker.
- [ ] Project actions panel polished.

### Milestone 0.4 — Developer Comfort

- [ ] Theme switching.
- [ ] Keybindings config.
- [ ] Git panel basic.
- [ ] Diff view.
- [ ] Search/replace project.
- [ ] Settings UI.

### Milestone 0.5 — Optional AI

- [ ] Provider config.
- [ ] Explain selection.
- [ ] Explain terminal error.
- [ ] Generate commit message.
- [ ] Review diff.
- [ ] Apply edit with diff confirmation.

---

## 18. Current Recommended Next Tasks

Kerjakan urutan ini dulu:

- [ ] Bootstrap repo Tauri + Svelte + Rust.
- [ ] Buat layout utama.
- [ ] Implement Open Folder.
- [ ] Implement File Explorer.
- [ ] Implement Monaco open/edit/save file.
- [ ] Implement tabs.
- [ ] Implement terminal basic.
- [ ] Implement project detector root scan.
- [ ] Implement project actions panel.

Jangan mulai dari AI, LSP lengkap, atau debugger sebelum editor lokalnya nyaman.
