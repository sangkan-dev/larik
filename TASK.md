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

- [ ] Buat repo `sangkan-dev/larik`.
- [ ] Tambahkan `README.md` awal.
- [ ] Tambahkan `PRD.md`.
- [ ] Tambahkan `TASK.md`.
- [ ] Tambahkan `AGENTS.md`.
- [ ] Tambahkan license `Apache-2.0 OR MIT`.
- [ ] Tambahkan `.gitignore` untuk Rust, Node, Tauri, OS files.
- [ ] Tambahkan `CONTRIBUTING.md` sederhana.
- [ ] Tambahkan `CODE_OF_CONDUCT.md` jika repo mulai dibuka untuk publik.

### 0.2 Tooling

- [ ] Setup Tauri v2 project.
- [ ] Setup Svelte + TypeScript.
- [ ] Setup Tailwind CSS.
- [ ] Setup formatter frontend.
- [ ] Setup formatter Rust.
- [ ] Setup linting frontend.
- [ ] Setup GitHub Actions basic CI.
- [ ] Setup build check untuk Linux.

### 0.3 Project Metadata

- [ ] Tentukan app name: `Larik`.
- [ ] Tentukan binary name: `larik`.
- [ ] Tentukan identifier aplikasi Tauri.
- [ ] Tambahkan app icon sementara.
- [ ] Tambahkan dark/light theme sementara.

---

## 1. Desktop Shell MVP

### 1.1 Tauri Window

- [ ] App bisa booting.
- [ ] Window title memakai `Larik`.
- [ ] Default window size nyaman untuk coding.
- [ ] Persist window size dan position.
- [ ] Tambahkan menu dasar.
- [ ] Tambahkan shortcut quit/reload/devtools untuk dev build.

### 1.2 Layout Dasar

- [ ] Buat layout utama:
  - [ ] activity bar / left rail minimal;
  - [ ] sidebar explorer;
  - [ ] editor area;
  - [ ] bottom panel;
  - [ ] status bar.
- [ ] Tambahkan resizable panels.
- [ ] Tambahkan collapse/expand sidebar.
- [ ] Tambahkan collapse/expand bottom panel.

### 1.3 State UI

- [ ] Buat store workspace aktif.
- [ ] Buat store tabs terbuka.
- [ ] Buat store active file.
- [ ] Buat store panel visibility.
- [ ] Simpan UI state lokal.

---

## 2. Workspace & File System

### 2.1 Open Folder

- [ ] Tambahkan command `Open Folder`.
- [ ] Buka native folder picker via Tauri.
- [ ] Simpan folder sebagai workspace aktif.
- [ ] Simpan daftar recent workspaces.
- [ ] Render nama folder di status bar/window title.

### 2.2 File Explorer

- [ ] Load file tree dari workspace.
- [ ] Ignore folder berat:
  - [ ] `.git`
  - [ ] `node_modules`
  - [ ] `vendor`
  - [ ] `target`
  - [ ] `dist`
  - [ ] `build`
- [ ] Expand/collapse folder.
- [ ] Open file dari explorer.
- [ ] Refresh explorer.
- [ ] Buat file baru.
- [ ] Buat folder baru.
- [ ] Rename file/folder.
- [ ] Delete file/folder dengan konfirmasi.

### 2.3 File Editing

- [ ] Read file content via Rust command.
- [ ] Tampilkan file di editor.
- [ ] Save file.
- [ ] Track dirty state.
- [ ] Prevent close tab jika file belum disimpan.
- [ ] Support binary/large file guard.
- [ ] Tampilkan pesan untuk file yang terlalu besar.

### 2.4 File Watcher

- [ ] Watch perubahan file workspace.
- [ ] Update explorer ketika file berubah.
- [ ] Deteksi file yang berubah di disk saat sedang dibuka.
- [ ] Tampilkan pilihan reload/keep local.

---

## 3. Editor Core via Monaco

### 3.1 Monaco Integration

- [ ] Install Monaco Editor.
- [ ] Render Monaco di editor area.
- [ ] Support multiple file models.
- [ ] Dispose model ketika tab ditutup.
- [ ] Set language mode berdasarkan file extension.
- [ ] Set theme dark/light.

### 3.2 Tabs

- [ ] Open file sebagai tab.
- [ ] Switch tab.
- [ ] Close tab.
- [ ] Close other tabs.
- [ ] Close all tabs.
- [ ] Reopen recently closed tab.
- [ ] Show dirty indicator.

### 3.3 Editor UX

- [ ] Basic find in file.
- [ ] Basic replace in file.
- [ ] Go to line.
- [ ] Minimap toggle.
- [ ] Word wrap toggle.
- [ ] Format document placeholder.
- [ ] Save shortcut.
- [ ] Save all shortcut.

### 3.4 Keybindings

- [ ] Implement keybinding registry awal.
- [ ] Tambahkan shortcut dasar:
  - [ ] Open folder
  - [ ] Save
  - [ ] Save all
  - [ ] Quick open
  - [ ] Command palette
  - [ ] Toggle terminal
  - [ ] Toggle sidebar
  - [ ] Close tab
- [ ] Buat format keybindings JSON sederhana.

---

## 4. Command Palette & Quick Open

### 4.1 Command Registry

- [ ] Buat command registry di frontend.
- [ ] Buat command metadata:
  - [ ] id
  - [ ] title
  - [ ] category
  - [ ] shortcut optional
  - [ ] handler
- [ ] Register command dasar editor.

### 4.2 Command Palette

- [ ] UI command palette.
- [ ] Fuzzy search command.
- [ ] Keyboard navigation.
- [ ] Execute selected command.
- [ ] Close on escape.

### 4.3 Quick Open File

- [ ] Index file path workspace.
- [ ] Fuzzy search file.
- [ ] Ignore folder berat.
- [ ] Open selected file.
- [ ] Keyboard navigation.

---

## 5. Integrated Terminal

### 5.1 Terminal UI

- [ ] Install xterm.js.
- [ ] Render terminal di bottom panel.
- [ ] Fit terminal size.
- [ ] Support terminal resize.
- [ ] Support terminal focus/blur.

### 5.2 Rust PTY Backend

- [ ] Implement spawn shell.
- [ ] Stream stdout/stderr ke frontend.
- [ ] Kirim keyboard input ke process.
- [ ] Resize PTY.
- [ ] Kill terminal process.
- [ ] Support multiple terminal sessions.

### 5.3 Terminal Commands

- [ ] Run command di workspace root.
- [ ] Buat terminal dari project action.
- [ ] Tampilkan command label.
- [ ] Auto-scroll output.
- [ ] Basic copy/paste.

---

## 6. Project Detector

### 6.1 Detector Core

- [ ] Buat module `project_detector` di Rust.
- [ ] Scan root workspace secara ringan.
- [ ] Return detected project types.
- [ ] Return confidence score sederhana.
- [ ] Return suggested actions.

### 6.2 Node / JS / TS Detector

- [ ] Deteksi `package.json`.
- [ ] Parse package manager:
  - [ ] npm
  - [ ] pnpm
  - [ ] yarn
  - [ ] bun
- [ ] Parse scripts dari `package.json`.
- [ ] Deteksi framework:
  - [ ] SvelteKit
  - [ ] Vite
  - [ ] Next.js
  - [ ] Express / generic Node
- [ ] Tampilkan action `install`, `dev`, `build`, `test` jika ada.

### 6.3 PHP / Laravel Detector

- [ ] Deteksi `composer.json`.
- [ ] Deteksi Laravel via `artisan`.
- [ ] Deteksi `app/`, `routes/`, `database/migrations`.
- [ ] Tampilkan action:
  - [ ] `composer install`
  - [ ] `php artisan serve`
  - [ ] `php artisan migrate`
  - [ ] `php artisan queue:work`
  - [ ] `php artisan test`
- [ ] Deteksi `.env.example` dan `.env`.

### 6.4 Rust Detector

- [ ] Deteksi `Cargo.toml`.
- [ ] Parse package name.
- [ ] Tampilkan action:
  - [ ] `cargo check`
  - [ ] `cargo run`
  - [ ] `cargo test`
  - [ ] `cargo build`

### 6.5 Go Detector

- [ ] Deteksi `go.mod`.
- [ ] Parse module name.
- [ ] Tampilkan action:
  - [ ] `go run .`
  - [ ] `go test ./...`
  - [ ] `go build ./...`

### 6.6 Docker Compose Detector

- [ ] Deteksi `docker-compose.yml`.
- [ ] Deteksi `compose.yml`.
- [ ] Tampilkan action:
  - [ ] `docker compose up`
  - [ ] `docker compose up -d`
  - [ ] `docker compose down`
  - [ ] `docker compose logs -f`
  - [ ] `docker compose ps`

### 6.7 Env Detector

- [ ] Deteksi `.env`.
- [ ] Deteksi `.env.example`.
- [ ] Compare missing keys.
- [ ] Tampilkan warning jika `.env` belum ada.
- [ ] Tampilkan daftar key yang belum terisi.

---

## 7. Project Actions Panel

### 7.1 UI

- [ ] Buat panel `Project`.
- [ ] Tampilkan detected project type.
- [ ] Tampilkan suggested actions.
- [ ] Tampilkan env status.
- [ ] Tampilkan scripts.

### 7.2 Action Runner

- [ ] Run action di integrated terminal.
- [ ] Support action working directory.
- [ ] Support action label.
- [ ] Tampilkan status running/done/failed.
- [ ] Prevent duplicate dangerous action jika perlu.

### 7.3 Safety

- [ ] Konfirmasi untuk action destructive:
  - [ ] delete file;
  - [ ] docker compose down;
  - [ ] migration destructive di masa depan.
- [ ] Jangan auto-run command setelah project dibuka.

---

## 8. Git Integration v0

### 8.1 Git Detection

- [ ] Deteksi apakah workspace adalah Git repo.
- [ ] Ambil branch aktif.
- [ ] Ambil status file berubah.
- [ ] Tampilkan indikator di status bar.

### 8.2 Git Panel

- [ ] Tampilkan changed files.
- [ ] Tampilkan staged/unstaged basic.
- [ ] Open changed file.
- [ ] Basic diff view placeholder.

### 8.3 Future Git Tasks

- [ ] Stage file.
- [ ] Unstage file.
- [ ] Commit.
- [ ] Generate commit message dengan AI optional.
- [ ] View diff inline.
- [ ] Git gutter.

---

## 9. LSP Integration

### 9.1 LSP Manager Skeleton

- [ ] Buat module `lsp` di Rust.
- [ ] Define language server config.
- [ ] Start language server process.
- [ ] Stop language server process.
- [ ] Send initialize request.
- [ ] Send shutdown request.
- [ ] Handle JSON-RPC messages.

### 9.2 Document Sync

- [ ] Send `textDocument/didOpen`.
- [ ] Send `textDocument/didChange`.
- [ ] Send `textDocument/didSave`.
- [ ] Send `textDocument/didClose`.

### 9.3 Diagnostics

- [ ] Receive diagnostics.
- [ ] Map diagnostics to file path.
- [ ] Show diagnostics in Monaco markers.
- [ ] Create Problems panel.

### 9.4 Completion

- [ ] Request completion from LSP.
- [ ] Map completion items to Monaco completion provider.
- [ ] Support resolve completion optional.

### 9.5 Hover & Navigation

- [ ] Hover provider.
- [ ] Go to definition.
- [ ] Find references.
- [ ] Rename symbol.
- [ ] Document formatting.
- [ ] Code action.

### 9.6 Initial Language Targets

- [ ] TypeScript / JavaScript.
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

- [ ] LSP manager basic.
- [ ] Diagnostics.
- [ ] Completion.
- [ ] Hover.
- [ ] Go to definition.
- [ ] Format on save.
- [ ] Problems panel.

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
