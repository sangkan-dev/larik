# DESIGN_SYSTEM — Larik Editor

> **Status:** Draft awal 0.1  
> **Produk:** Larik / Larik Editor  
> **Tujuan dokumen:** Menjadi panduan visual, UI, UX, dan implementasi komponen untuk Larik agar konsisten saat dikerjakan manusia maupun AI coding agent.

---

## 1. Prinsip Desain

Larik bukan editor yang ingin terlihat futuristik berlebihan. Larik harus terasa seperti alat kerja developer: ringan, fokus, tenang, dan bisa dipercaya.

### 1.1 Karakter Produk

```txt
Ringan        : terasa cepat, tidak ramai, tidak banyak dekorasi.
Project-aware : UI membantu user memahami project, bukan cuma membuka file.
Praktis       : fitur utama mudah ditemukan, bukan tersembunyi di banyak extension.
Tenang        : tidak terlalu neon, tidak terlalu AI-looking, tidak terlalu gaming.
Terbuka       : cocok untuk open-source, mudah dikembangkan komunitas.
```

### 1.2 Kalimat Arah Visual

> **A calm, lightweight code workspace for developers who want to code, run, fix, and deploy projects without extension bloat.**

Versi Indonesia:

> **Workspace coding yang ringan, tenang, dan ngerti project.**

---

## 2. Design Goals

### 2.1 Tujuan Utama

- Membuat editor terasa cepat, bersih, dan tidak melelahkan untuk dipakai lama.
- Memudahkan user memahami struktur project, command yang tersedia, status environment, dan masalah umum.
- Mengurangi kebutuhan extension melalui fitur bawaan yang cukup untuk workflow coding umum.
- Menyediakan AI sebagai fitur opsional, bukan pusat visual produk.
- Memastikan desain mudah diimplementasikan di Svelte + Tailwind.

### 2.2 Non-Goals

- Tidak mengejar tampilan seperti Cursor/Windsurf yang sangat AI-first.
- Tidak membuat UI terlalu enterprise/corporate.
- Tidak membuat theme default terlalu ramai atau terlalu kontras.
- Tidak membuat layout yang terlalu berbeda dari kebiasaan developer sampai membingungkan.
- Tidak meniru 1:1 VS Code, walaupun beberapa pola UX boleh familiar.

---

## 3. Brand Foundation

### 3.1 Nama

**Larik** berarti baris, deret, atau susunan. Dalam konteks code editor, Larik merepresentasikan baris kode, alur kerja, dan struktur project.

### 3.2 Personality

```txt
Tenang      : tidak banyak gimmick.
Cerdas      : membantu lewat konteks project, bukan pop-up berlebihan.
Teknis      : tetap terasa sebagai alat developer.
Membumi     : cocok untuk developer Indonesia, mahasiswa, komunitas, dan tim kecil.
Open-source : tidak terlalu polished-corporate sampai terasa tertutup.
```

### 3.3 Tone Copywriting

Gunakan bahasa yang jelas dan praktis.

Contoh baik:

```txt
Project Laravel terdeteksi.
File .env belum ditemukan.
Port 5173 sedang dipakai proses lain.
Jalankan npm install sebelum npm run dev.
AI tidak aktif. Tambahkan provider jika ingin memakai bantuan AI.
```

Hindari:

```txt
Oopsie! Something magical happened.
Your AI superpower is ready.
Let the agent handle everything.
```

---

## 4. Visual Direction

### 4.1 Style Keywords

```txt
minimal
calm
sharp
local-first
developer tool
monochrome base
warm accent
low distraction
```

### 4.2 Mood

Larik harus terasa seperti perpaduan:

```txt
- terminal tool yang rapi;
- modern code editor;
- dashboard project kecil;
- bukan AI SaaS landing page.
```

### 4.3 Hindari Visual Ini

- Gradient ungu-biru berlebihan khas AI product.
- Glassmorphism berat.
- Banyak glow/neon.
- Animasi terlalu playful.
- Sidebar penuh warna tanpa hierarchy.
- Banyak card marketing di dalam editor.

---

## 5. Color System

Larik menggunakan warna dasar netral gelap/terang dengan aksen teal dan amber. Teal dipakai untuk fokus dan status aktif. Amber dipakai untuk warning, attention, dan project hints.

### 5.1 Semantic Tokens

Gunakan semantic token di UI, jangan hardcode warna langsung di komponen.

```txt
background
surface
surface-muted
surface-raised
border
border-muted
text
text-muted
text-subtle
accent
accent-hover
accent-muted
warning
success
danger
info
focus-ring
selection
```

### 5.2 Dark Theme — Default

Dark theme menjadi default karena mayoritas code editor dipakai lama dan banyak developer memilih mode gelap.

```css
:root.dark {
  --background: #0b0f10;
  --surface: #101617;
  --surface-muted: #151d1f;
  --surface-raised: #1b2427;

  --border: #263236;
  --border-muted: #1c272a;

  --text: #e6edf0;
  --text-muted: #a8b4b8;
  --text-subtle: #6f7d82;

  --accent: #2dd4bf;
  --accent-hover: #5eead4;
  --accent-muted: #134e4a;

  --warning: #f59e0b;
  --success: #22c55e;
  --danger: #ef4444;
  --info: #38bdf8;

  --focus-ring: #2dd4bf;
  --selection: #164e63;
}
```

### 5.3 Light Theme

```css
:root.light {
  --background: #f7f8f8;
  --surface: #ffffff;
  --surface-muted: #f0f3f3;
  --surface-raised: #ffffff;

  --border: #d7dfdf;
  --border-muted: #e6ecec;

  --text: #172022;
  --text-muted: #526165;
  --text-subtle: #7a898d;

  --accent: #0f766e;
  --accent-hover: #0d9488;
  --accent-muted: #ccfbf1;

  --warning: #d97706;
  --success: #16a34a;
  --danger: #dc2626;
  --info: #0284c7;

  --focus-ring: #0f766e;
  --selection: #bae6fd;
}
```

### 5.4 Usage Rules

```txt
Teal:
- active tab indicator
- primary button
- focus ring
- active command
- project detected status

Amber:
- env missing
- warning state
- non-blocking issue
- recommendation hint

Red:
- destructive action
- failed command
- build error

Green:
- success command
- service healthy
- deploy ready

Blue:
- info
- link
- docs/helper
```

---

## 6. Typography

### 6.1 UI Font

Rekomendasi:

```txt
Primary UI Font : Inter
Fallback        : system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif
```

Alternatif:

```txt
Geist Sans
Manrope
```

### 6.2 Code Font

Rekomendasi:

```txt
Primary Code Font : JetBrains Mono
Fallback          : "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace
```

Alternatif:

```txt
Fira Code
Cascadia Code
IBM Plex Mono
```

### 6.3 Type Scale

```txt
caption     : 11px / 16px
body-sm     : 12px / 18px
body        : 13px / 20px
body-lg     : 14px / 22px
heading-sm  : 15px / 22px
heading     : 16px / 24px
heading-lg  : 18px / 28px
```

Editor UI sebaiknya tidak terlalu besar. Default body 13px cukup ideal untuk density code editor.

### 6.4 Font Weight

```txt
regular  : 400
medium   : 500
semibold : 600
bold     : 700, jarang dipakai
```

Gunakan `medium` untuk label aktif, bukan selalu bold.

---

## 7. Spacing & Radius

### 7.1 Spacing Scale

```txt
--space-1 : 4px
--space-2 : 8px
--space-3 : 12px
--space-4 : 16px
--space-5 : 20px
--space-6 : 24px
--space-8 : 32px
```

### 7.2 Border Radius

```txt
xs : 4px
sm : 6px
md : 8px
lg : 12px
xl : 16px
```

Rules:

```txt
- Button: 6px atau 8px.
- Panel/card: 8px atau 12px.
- Input: 6px.
- Editor tab: 6px top radius jika diperlukan.
```

Jangan terlalu rounded agar tetap terasa seperti developer tool.

---

## 8. Layout System

### 8.1 Main App Layout

Default layout Larik:

```txt
┌─────────────────────────────────────────────────────────┐
│ Top Bar / Command Area                                  │
├───────────────┬───────────────────────────┬─────────────┤
│ Activity Bar  │ Editor Area               │ Right Panel │
│ + Sidebar     │                           │ optional    │
│               ├───────────────────────────┤             │
│               │ Bottom Panel / Terminal   │             │
├───────────────┴───────────────────────────┴─────────────┤
│ Status Bar                                               │
└─────────────────────────────────────────────────────────┘
```

### 8.2 Layout Principles

- Sidebar kiri untuk Explorer, Search, Git, Project.
- Editor area tetap menjadi pusat.
- Bottom panel untuk Terminal, Problems, Output.
- Right panel hanya untuk fitur kontekstual seperti AI/chat/project insight, dan boleh hidden by default.
- Status bar tipis untuk branch, language mode, LSP status, line/column, formatter, encoding.

### 8.3 Panel Width

```txt
Activity bar       : 44px
Sidebar default    : 260px
Sidebar min        : 200px
Sidebar max        : 420px
Right panel default: 360px
Bottom panel       : 220px - 320px
Status bar         : 24px
Top bar            : 36px - 44px
```

---

## 9. Component System

### 9.1 Button

Variants:

```txt
primary
secondary
ghost
danger
link
```

Sizes:

```txt
sm : 24px height
md : 32px height
lg : 40px height
```

Rules:

```txt
- Primary hanya untuk aksi utama.
- Ghost untuk toolbar/editor action.
- Danger hanya untuk destructive action seperti delete workspace/config.
- Button harus punya focus state.
```

Example Tailwind-style:

```svelte
<button
  class="h-8 rounded-md border border-border bg-surface px-3 text-sm text-text hover:bg-surface-muted focus:outline-none focus:ring-2 focus:ring-focus-ring"
>
  Run Project
</button>
```

### 9.2 Input

Untuk search, command palette, env editor, settings.

Rules:

```txt
- Height default 32px.
- Border subtle.
- Focus ring teal.
- Placeholder text-subtle.
- Error pakai border danger + helper text.
```

### 9.3 Tabs

Tab editor harus compact.

States:

```txt
inactive
active
dirty
pinned
preview
```

Rules:

```txt
- Active tab pakai border/accent indicator, bukan background terlalu mencolok.
- Dirty state pakai dot kecil.
- Close icon muncul saat hover atau active.
```

### 9.4 Sidebar Item

States:

```txt
normal
hover
active
modified
error
ignored
```

Rules:

```txt
- Folder/file tree harus dense.
- Indentation jelas.
- Icon jangan terlalu warna-warni di default theme.
- Modified file bisa pakai text warning atau dot kecil.
```

### 9.5 Command Palette

Command palette adalah salah satu UX utama.

Shortcut default:

```txt
Ctrl+Shift+P / Cmd+Shift+P : Open Command Palette
Ctrl+P / Cmd+P             : Quick Open File
```

Layout:

```txt
- floating centered panel
- width 640px
- max-height 70vh
- input di atas
- result list di bawah
```

Command groups:

```txt
File
Workspace
Project
Terminal
Git
LSP
AI
Sakala
Settings
```

AI command harus tetap ada, tapi tidak dominan.

### 9.6 Project Card / Project Header

Dipakai di project panel.

Informasi:

```txt
- project name
- detected stack
- root path
- git branch
- package manager/runtime
- env status
- available actions
```

Contoh:

```txt
Project: laravel-app
Stack: Laravel + Vite + Docker Compose
Status: .env missing, database unknown
Actions: composer install, npm install, artisan serve, migrate
```

### 9.7 Problems Panel

Sources:

```txt
- LSP diagnostics
- command parser
- env checker
- project detector
```

Severity:

```txt
error
warning
info
hint
```

Rules:

```txt
- Error harus mudah dibuka ke file/baris.
- Warning jangan terlalu agresif.
- Hints bisa collapsed by default.
```

### 9.8 Terminal Panel

Terminal harus terasa native dan tidak terlalu banyak chrome.

Rules:

```txt
- Tab terminal minimal.
- Aksi: new, split later, kill, clear.
- Command runner output bisa masuk terminal atau output panel.
- Error summary boleh muncul sebagai action manual.
```

### 9.9 Toast / Notification

Gunakan hemat.

Types:

```txt
success
error
warning
info
```

Rules:

```txt
- Toast tidak boleh menutup coding area terlalu lama.
- Untuk error command, prefer Problems/Output daripada toast panjang.
- Toast maksimal 1-2 baris.
```

---

## 10. Iconography

### 10.1 Icon Library

Rekomendasi awal:

```txt
lucide-svelte
```

Kenapa:

```txt
- ringan
- open-source
- style konsisten
- mudah dipakai di Svelte
```

### 10.2 Icon Style

```txt
stroke width : 1.75 - 2px
size toolbar : 16px
size sidebar : 16px
size empty state : 32px - 48px
```

### 10.3 File Icons

Untuk MVP:

```txt
- generic file
- folder
- folder open
- js/ts
- svelte
- php
- rust
- go
- python
- docker
- yaml
- markdown
- json
- git
```

Jangan dulu mengejar icon theme full seperti VS Code.

---

## 11. Syntax Highlight Theme Direction

Default syntax theme harus low-distraction.

### 11.1 Dark Syntax Palette

```txt
keyword      : #5eead4
string       : #a7f3d0
number       : #fbbf24
function     : #93c5fd
variable     : #e6edf0
property     : #c4b5fd
comment      : #64748b
operator     : #94a3b8
type/class   : #fca5a5
constant     : #fdba74
invalid      : #ef4444
```

### 11.2 Rules

- Jangan terlalu banyak warna mencolok.
- Comment harus lebih subtle.
- Error/invalid harus jelas.
- Selection dan current line jangan terlalu terang.

---

## 12. Motion & Interaction

### 12.1 Motion Principles

```txt
- cepat
- subtle
- tidak mengganggu typing
- tidak ada animasi besar di editor area
```

### 12.2 Durations

```txt
fast   : 80ms
normal : 120ms
slow   : 180ms
```

### 12.3 Allowed Motion

```txt
- hover transition
- panel open/close subtle
- command palette fade/scale kecil
- toast slide/fade
```

### 12.4 Avoid

```txt
- loading spinner berlebihan
- typing animation untuk AI
- animated gradient
- sidebar bounce
```

---

## 13. Empty States

Empty state harus membantu, bukan marketing.

### 13.1 No Folder Open

```txt
Belum ada folder dibuka.
Buka folder project untuk mulai coding, menjalankan command, dan melihat struktur project.

[Open Folder]
```

### 13.2 No Problems

```txt
Tidak ada masalah terdeteksi.
Diagnostics dari language server dan project checker akan muncul di sini.
```

### 13.3 AI Not Configured

```txt
AI belum aktif.
Larik tetap bisa dipakai penuh tanpa AI. Tambahkan provider jika ingin memakai perintah seperti explain error atau review diff.

[Configure Provider]
```

### 13.4 No Terminal

```txt
Belum ada terminal aktif.
Buat terminal baru untuk menjalankan command project.

[New Terminal]
```

---

## 14. UX Patterns

### 14.1 Project Detection Flow

```txt
User membuka folder
  -> Larik scan file root
  -> detect stack
  -> tampilkan Project Panel
  -> tawarkan action relevan
```

Contoh Laravel:

```txt
Laravel project detected
- composer install
- npm install
- copy .env.example to .env
- php artisan key:generate
- php artisan migrate
- php artisan serve
```

### 14.2 Command Runner Flow

```txt
User memilih command
  -> tampilkan command preview
  -> user confirm/run
  -> output masuk terminal/output panel
  -> exit code dibaca
  -> jika gagal, tampilkan Problems/Output summary
```

Jangan auto-run command berisiko tanpa konfirmasi.

### 14.3 AI Flow

```txt
User memilih teks/error/diff
  -> user menjalankan AI command manual
  -> Larik tampilkan context yang akan dikirim
  -> user confirm jika provider cloud
  -> hasil tampil sebagai suggestion, bukan auto-apply
```

Rules:

```txt
- AI tidak aktif by default.
- AI tidak boleh mengirim seluruh workspace otomatis.
- AI tidak boleh auto-apply perubahan tanpa review.
```

---

## 15. Accessibility

Minimum requirement:

```txt
- semua interactive element bisa diakses keyboard
- focus ring jelas
- kontras teks cukup
- command palette keyboard-first
- tab order masuk akal
- icon penting punya label/tooltip
- jangan mengandalkan warna saja untuk status
```

Status harus punya teks atau icon tambahan:

```txt
Error   : icon + text
Warning : icon + text
Success : icon + text
```

---

## 16. Responsive / Window Sizes

Larik adalah desktop app, tapi tetap harus nyaman di beberapa ukuran window.

### 16.1 Minimum Practical Size

```txt
width  : 1024px
height : 640px
```

### 16.2 Behavior

```txt
- Sidebar bisa collapse.
- Bottom panel bisa close.
- Right panel hidden by default.
- Activity bar tetap compact.
- Command palette tetap centered.
```

---

## 17. Design Tokens for Tailwind

Contoh mapping awal `tailwind.config.ts`:

```ts
export default {
  theme: {
    extend: {
      colors: {
        background: "var(--background)",
        surface: "var(--surface)",
        "surface-muted": "var(--surface-muted)",
        "surface-raised": "var(--surface-raised)",
        border: "var(--border)",
        "border-muted": "var(--border-muted)",
        text: "var(--text)",
        "text-muted": "var(--text-muted)",
        "text-subtle": "var(--text-subtle)",
        accent: "var(--accent)",
        "accent-hover": "var(--accent-hover)",
        "accent-muted": "var(--accent-muted)",
        warning: "var(--warning)",
        success: "var(--success)",
        danger: "var(--danger)",
        info: "var(--info)",
        "focus-ring": "var(--focus-ring)",
        selection: "var(--selection)",
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "SFMono-Regular", "Consolas", "monospace"],
      },
      borderRadius: {
        xs: "4px",
        sm: "6px",
        md: "8px",
        lg: "12px",
        xl: "16px",
      },
    },
  },
};
```

---

## 18. Component Naming Convention

Svelte component naming:

```txt
AppShell.svelte
TopBar.svelte
ActivityBar.svelte
Sidebar.svelte
EditorTabs.svelte
EditorPane.svelte
BottomPanel.svelte
StatusBar.svelte
CommandPalette.svelte
ProjectPanel.svelte
ProblemsPanel.svelte
TerminalPanel.svelte
SettingsView.svelte
```

UI primitive naming:

```txt
Button.svelte
IconButton.svelte
Input.svelte
Select.svelte
Tooltip.svelte
Dialog.svelte
Panel.svelte
Tabs.svelte
Badge.svelte
Toast.svelte
```

---

## 19. Information Architecture

### 19.1 Activity Bar Items

Urutan awal:

```txt
Explorer
Search
Project
Git
Terminal
AI
Settings
```

AI jangan ditempatkan paling atas agar tidak terasa AI-first.

### 19.2 Status Bar Items

Kiri:

```txt
Git branch
Project type
Command status
```

Kanan:

```txt
LSP status
Formatter
Line/column
Encoding
Theme mode
```

---

## 20. Keyboard Shortcuts

Default shortcuts mengikuti kebiasaan umum editor.

```txt
Ctrl/Cmd + P           : Quick Open
Ctrl/Cmd + Shift + P   : Command Palette
Ctrl/Cmd + `           : Toggle Terminal
Ctrl/Cmd + B           : Toggle Sidebar
Ctrl/Cmd + S           : Save
Ctrl/Cmd + Shift + F   : Search in Project
Ctrl/Cmd + ,           : Settings
F12                    : Go to Definition
Shift + F12            : Find References
F2                     : Rename Symbol
```

Semua shortcut harus bisa diubah di masa depan.

---

## 21. Naming for Features

Gunakan nama fitur yang jelas.

```txt
Project Panel
Run Actions
Problems
Output
Terminal
Quick Open
Command Palette
AI Commands
Deploy Checklist
Env Checker
```

Hindari nama terlalu marketing di dalam app seperti:

```txt
Magic Fixer
Super Agent
Code Genius
Autopilot
```

---

## 22. First-Run Experience

Flow pertama kali:

```txt
1. App terbuka tanpa login.
2. User melihat empty state sederhana.
3. CTA utama: Open Folder.
4. Setelah folder dibuka, Larik mendeteksi project.
5. Project Panel menampilkan stack dan action relevan.
```

Tidak ada:

```txt
- wajib login;
- wajib pilih AI provider;
- onboarding panjang;
- newsletter/signup;
- marketplace prompt.
```

---

## 23. Larik Default Theme Names

Theme bawaan:

```txt
Larik Dark
Larik Light
Larik Dimmed
```

Opsional nanti:

```txt
Larik Terminal
Larik Warm
Larik High Contrast
```

---

## 24. Implementation Checklist

### v0.1 Design Scope

- [ ] App shell layout.
- [ ] Dark theme default.
- [ ] Light theme basic.
- [ ] Activity bar.
- [ ] Sidebar explorer.
- [ ] Editor tabs.
- [ ] Monaco editor styling.
- [ ] Bottom panel.
- [ ] Status bar.
- [ ] Command palette basic.
- [ ] Button/Input/Panel primitives.
- [ ] Project empty state.
- [ ] Terminal empty state.

### v0.2 Design Scope

- [ ] Problems panel.
- [ ] Project panel.
- [ ] Settings view.
- [ ] Theme switcher.
- [ ] Toast system.
- [ ] Git status indicators.
- [ ] LSP diagnostics style.
- [ ] Run command UX.

### v0.3 Design Scope

- [ ] AI command panel.
- [ ] Provider settings.
- [ ] Diff review UI.
- [ ] Env checker UI.
- [ ] Deploy checklist UI.
- [ ] Sakala integration UI draft.

---

## 25. Design Review Rules for Contributors

Sebelum merge UI change, cek:

```txt
- Apakah UI tetap ringan dan tidak ramai?
- Apakah bisa dipakai tanpa AI?
- Apakah keyboard navigation tetap jalan?
- Apakah warna memakai design token?
- Apakah spacing konsisten?
- Apakah komponen baru benar-benar perlu?
- Apakah copywriting jelas dan tidak terlalu marketing?
- Apakah fitur tidak membuat editor terasa seperti dashboard SaaS?
```

---

## 26. Summary

Larik memakai design system yang tenang, ringan, dan praktis. Fokus visualnya adalah membantu developer bekerja lebih cepat tanpa gangguan: buka project, pahami struktur, edit kode, jalankan command, lihat error, dan siapkan deploy.

AI boleh menjadi fitur tambahan, tetapi desain Larik tidak boleh bergantung pada AI. Identitas utama Larik adalah:

> **Lightweight, project-aware, local-first code editor.**
