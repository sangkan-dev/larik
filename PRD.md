# PRD — Larik Editor

> **Status:** Draft awal 0.1  
> **Produk:** Larik / Larik Editor  
> **Repo target:** `sangkan-dev/larik`  
> **Tipe:** Open-source lightweight project-aware code editor  
> **Stack awal:** Tauri v2, Rust, Svelte, TypeScript, Monaco Editor, xterm.js, LSP, Tree-sitter  

---

## 1. Ringkasan Produk

**Larik** adalah code editor ringan, open-source, dan project-aware untuk developer modern yang ingin langsung menulis, menjalankan, memperbaiki, dan menyiapkan project untuk deploy tanpa harus bergantung pada banyak extension atau AI yang mahal.

Larik bukan VS Code clone dan bukan Cursor clone. Larik mengambil pendekatan yang lebih fokus:

- ringan dan cepat dibuka;
- nyaman untuk coding sehari-hari;
- mendukung banyak bahasa melalui LSP;
- paham struktur project populer seperti Laravel, Svelte, Node, Rust, Go, Docker, dan YAML;
- punya terminal dan command runner bawaan;
- AI bersifat opsional, hemat, dan manual-trigger;
- ke depannya bisa terintegrasi dengan workflow deploy seperti Sakala.

### Positioning

> **Lightweight project-aware code editor for coding, running, fixing, and deploying projects without extension bloat.**

Versi singkat:

> **Editor ringan yang ngerti project, bukan cuma file.**

---

## 2. Latar Belakang

Developer saat ini banyak memakai VS Code karena ekosistem extension-nya kuat. Namun, extension yang terlalu banyak sering membuat editor berat, memakan RAM, dan membuat pengalaman kerja bergantung pada konfigurasi yang tidak selalu konsisten.

Di sisi lain, tren AI editor sedang naik, tetapi pendekatan AI-first berisiko mahal, terutama untuk target pengguna di Indonesia seperti mahasiswa, developer pemula, komunitas, tim kecil, dan developer internal perusahaan yang sensitif biaya.

Larik mengambil jalan tengah:

- fitur inti editor harus tetap kuat tanpa AI;
- AI boleh ada, tetapi tidak menjadi syarat utama agar editor berguna;
- integrasi bahasa dilakukan lewat standar seperti LSP, bukan extension runtime kompleks;
- project-awareness dan DevOps workflow menjadi pembeda utama.

---

## 3. Masalah yang Diselesaikan

### 3.1 Extension Bloat

Banyak developer harus menginstall banyak extension untuk mendapatkan pengalaman coding yang nyaman.

Contoh:

- extension bahasa;
- formatter;
- snippets;
- Docker helper;
- Laravel helper;
- Git helper;
- terminal helper;
- AI helper.

Larik ingin mengurangi kebutuhan extension dengan menyediakan fitur bawaan yang cukup untuk workflow umum.

### 3.2 Editor Berat untuk Laptop Biasa

Tidak semua developer memakai laptop high-end. Banyak pengguna memakai laptop kantor, laptop second, RAM 8–16 GB, atau Linux desktop sederhana.

Larik harus nyaman di perangkat biasa.

### 3.3 AI Mahal Jika Jadi Fondasi Utama

AI membantu, tetapi token cost bisa menjadi masalah. Larik tidak boleh bergantung pada AI untuk fitur dasar seperti autocomplete, diagnostics, formatting, dan project detection.

### 3.4 Editor Tidak Paham Workflow Deploy

Banyak editor fokus pada file editing, tetapi developer sering juga harus menjalankan service, membaca log, mengecek env, build Docker image, dan deploy.

Larik ingin menjadi editor yang lebih dekat ke workflow:

```txt
code -> run -> inspect -> fix -> deploy
```

---

## 4. Target Pengguna

### 4.1 Primary Users

1. **Developer pemula / mahasiswa**
   - butuh editor yang langsung bisa dipakai;
   - sering bingung setup extension, formatter, env, dan run command;
   - butuh error message yang lebih membumi.

2. **Backend / fullstack developer**
   - bekerja dengan Laravel, Node, Svelte, Docker, PostgreSQL, Go, Rust;
   - butuh terminal, logs, LSP, formatter, dan Git dalam satu workspace.

3. **Developer Linux / low-resource machine user**
   - butuh editor yang ringan;
   - tidak ingin Electron-heavy workflow;
   - ingin tool open-source yang bisa dioprek.

4. **Tim kecil / komunitas / internal perusahaan**
   - ingin editor yang simple;
   - AI bisa memakai provider sendiri;
   - workflow bisa disesuaikan dengan kebutuhan internal.

### 4.2 Secondary Users

1. Maintainer open-source yang ingin kontribusi ke editor.
2. DevOps engineer yang ingin workflow code + container + deploy lebih rapat.
3. Pengguna Sakala yang ingin deploy project dari editor.

---

## 5. Prinsip Produk

### 5.1 Lightweight First

Larik harus terasa ringan dari awal:

- startup cepat;
- RAM relatif hemat;
- indexing tidak agresif;
- background process transparan;
- tidak menjalankan AI otomatis.

### 5.2 LSP-first, Not Extension-first

Larik tidak mengejar kompatibilitas penuh dengan VS Code extension. Fokus awal adalah dukungan bahasa melalui Language Server Protocol.

### 5.3 Project-aware, Not Just File-aware

Larik harus mampu mengenali jenis project dan memberikan action relevan.

Contoh:

- Laravel: artisan, composer, env, migration, queue;
- SvelteKit: npm scripts, dev server, routes;
- Rust: cargo check, test, run;
- Go: go run, go test, module info;
- Docker Compose: services, logs, up/down.

### 5.4 AI Optional and Cost-controlled

AI tidak boleh otomatis membakar token. AI harus:

- off by default;
- manual trigger;
- BYOK-friendly;
- mendukung OpenAI-compatible endpoint;
- mendukung local endpoint seperti Ollama/vLLM di fase lanjut;
- mengirim konteks kecil dan relevan.

### 5.5 Offline-first

Larik harus tetap berguna tanpa internet:

- buka/edit file;
- terminal;
- LSP lokal;
- formatter lokal;
- Git lokal;
- project detection lokal.

### 5.6 Open-source Friendly

Kode harus mudah dibaca, struktur modular, dan kontribusi tidak terlalu sulit.

---

## 6. Non-goals

Fitur berikut **bukan target awal**:

- kompatibilitas penuh VS Code extension;
- marketplace extension sendiri;
- remote SSH seperti VS Code;
- cloud IDE penuh;
- real-time collaboration;
- debugger lengkap sejak v0.1;
- AI agent otomatis yang memodifikasi project tanpa konfirmasi;
- custom editor core dari nol di fase awal;
- menggantikan semua fitur VS Code.

---

## 7. Scope MVP

### 7.1 MVP Objective

MVP dianggap berhasil jika Larik bisa digunakan untuk membuka project lokal, mengedit file, menjalankan terminal, mengenali tipe project, dan memberi pengalaman coding dasar yang nyaman.

### 7.2 MVP Features

#### Core Workspace

- Open folder.
- File explorer.
- Open/edit/save file.
- Multi-tab editor.
- Basic command palette.
- Quick open file.
- Project search sederhana.

#### Editor

- Monaco Editor sebagai editor awal.
- Syntax highlight dasar.
- Basic keyboard shortcuts.
- Dark theme dan light theme bawaan.

#### Terminal

- Integrated terminal.
- Spawn shell lokal.
- Resize terminal.
- Run command dari UI.

#### Project Detection

- Deteksi `package.json`.
- Deteksi `composer.json`.
- Deteksi `Cargo.toml`.
- Deteksi `go.mod`.
- Deteksi `docker-compose.yml` / `compose.yml`.
- Deteksi `.env` dan `.env.example`.

#### Project Actions

- Tampilkan npm scripts.
- Tampilkan composer scripts jika ada.
- Tampilkan cargo commands dasar.
- Tampilkan go commands dasar.
- Tampilkan Docker Compose actions dasar.

#### LSP v0

- LSP manager skeleton.
- Start/stop language server.
- Diagnostics minimal.
- Completion minimal untuk satu bahasa awal.

#### Git v0

- Deteksi Git repo.
- Tampilkan branch aktif.
- Tampilkan daftar file berubah.

---

## 8. Future Scope

### 8.1 Language Experience

- Hover.
- Go to definition.
- Find references.
- Rename symbol.
- Format on save.
- Code action.
- Problems panel.

### 8.2 Tree-sitter Layer

- Incremental parsing.
- Symbol outline.
- Folding improvement.
- Better syntax-aware selection.
- Project indexing ringan.

### 8.3 Debugging

- DAP client.
- Breakpoints.
- Variables panel.
- Call stack.
- Debug console.

### 8.4 AI Optional

- BYOK provider.
- OpenAI-compatible endpoint.
- Ollama/local endpoint.
- Explain selected code.
- Explain terminal error.
- Generate commit message.
- Review diff.
- Generate Dockerfile.
- Generate `.env.example` suggestion.

### 8.5 Sakala Integration

- Detect deploy readiness.
- Preview build plan.
- Deploy to Sakala.
- View deployment logs.
- Open public URL.
- Rollback deployment.

---

## 9. Technical Stack

### 9.1 Final Stack Decision

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
Debug           : DAP client later
Git             : git CLI first
Config          : TOML + JSON compatibility
State           : local files first, SQLite later if needed
AI              : optional BYOK/OpenAI-compatible/Ollama later
License         : Apache-2.0 OR MIT
```

### 9.2 Architecture

```txt
┌────────────────────────────────────────────┐
│                  Larik UI                  │
│        Svelte + TypeScript + Tailwind      │
│                                            │
│  Explorer | Editor | Terminal | Problems   │
└───────────────────┬────────────────────────┘
                    │ Tauri IPC
┌───────────────────▼────────────────────────┐
│              Larik Core Rust               │
│                                            │
│  Workspace Manager                         │
│  File System / Watcher                     │
│  Project Detector                          │
│  Command Runner                            │
│  PTY Terminal Backend                      │
│  LSP Client Manager                        │
│  Tree-sitter Indexer                       │
│  Settings Manager                          │
│  Git Adapter                               │
└───────────────────┬────────────────────────┘
                    │
     ┌──────────────┼──────────────┐
     │              │              │
┌────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐
│   LSP    │  │  Shell    │  │   Git     │
│ Servers  │  │ Process   │  │   CLI     │
└──────────┘  └───────────┘  └───────────┘
```

---

## 10. Suggested Repository Structure

### 10.1 MVP Structure

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

### 10.2 Future Modular Structure

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
  docs/
    architecture.md
    roadmap.md
    contributing.md
```

---

## 11. Product Experience

### 11.1 First Run

User membuka Larik dan melihat:

- tombol Open Folder;
- recent workspaces;
- shortcut dokumentasi;
- tidak wajib login;
- tidak wajib setup AI.

### 11.2 Open Project

Ketika folder dibuka, Larik melakukan:

- scan ringan file root;
- deteksi tipe project;
- tampilkan project summary;
- tampilkan suggested actions.

Contoh untuk Laravel:

```txt
Project detected: Laravel

Actions:
- composer install
- php artisan serve
- php artisan migrate
- npm run dev
- open .env
- check env
```

### 11.3 Coding Flow

User bisa:

- buka file dari explorer;
- quick open file;
- edit dan save;
- lihat diagnostics;
- jalankan terminal;
- run project action;
- lihat status Git.

---

## 12. Success Metrics

### 12.1 MVP Success

- Bisa buka folder dan edit file stabil.
- Terminal bisa digunakan untuk command harian.
- Project detector bekerja untuk minimal Node, Laravel, Rust, Go, Docker Compose.
- LSP skeleton berjalan untuk minimal satu bahasa.
- Build desktop app berhasil untuk Linux minimal.

### 12.2 Product Success

- Developer bisa memakai Larik untuk coding project sederhana tanpa VS Code.
- Setup awal tidak butuh extension.
- AI tidak diperlukan untuk workflow dasar.
- Resource usage terasa lebih ringan dibanding editor Electron-heavy.
- Kontributor open-source bisa memahami struktur repo dengan cepat.

---

## 13. Risks

### 13.1 Monaco Dependency

Monaco mempercepat MVP tetapi membawa dependency web editor. Risiko: Larik belum sepenuhnya native/lightweight.

Mitigasi:

- pakai Monaco dulu untuk validasi;
- siapkan abstraksi editor adapter;
- evaluasi custom editor core di fase lanjut.

### 13.2 Scope Creep

Editor mudah melebar menjadi VS Code clone.

Mitigasi:

- disiplin pada non-goals;
- semua fitur harus mendukung flow code-run-fix-deploy;
- hindari extension marketplace di fase awal.

### 13.3 LSP Complexity

LSP client bisa kompleks karena tiap language server punya perilaku berbeda.

Mitigasi:

- mulai dari satu bahasa;
- buat abstraction layer sederhana;
- tambahkan adapter per bahasa secara bertahap.

### 13.4 Cross-platform Complexity

PTY, file watcher, shell, path, dan terminal behavior berbeda antar OS.

Mitigasi:

- target awal Linux;
- Windows/macOS setelah core stabil;
- gunakan crate/library yang cross-platform jika memungkinkan.

---

## 14. Open Questions

- Apakah target awal hanya Linux atau langsung Linux/macOS/Windows?
- Apakah Larik akan punya website sendiri seperti `larik.dev` atau cukup GitHub repo?
- Apakah integrasi Sakala masuk v0.2 atau dibiarkan sebagai plugin/future module?
- Apakah settings format utama TOML, JSON, atau keduanya?
- Apakah theme awal dibuat sendiri atau import subset dari VS Code theme JSON?

---

## 15. Recommended First Milestone

Milestone pertama jangan mengejar LSP lengkap. Fokus ke editor yang bisa dipakai membuka dan mengedit project.

```txt
Milestone 0.1 — Usable Local Editor

- Tauri app booting
- Open folder
- File explorer
- Open/edit/save file
- Monaco editor
- Multi-tab basic
- Integrated terminal basic
- Project detector root files
- Basic project actions
- Git branch and changed files
```

Setelah itu baru masuk LSP dan project presets yang lebih rapi.
