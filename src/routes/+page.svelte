<script lang="ts">
  const setupItems = [
    "Tauri v2 desktop shell",
    "Svelte + TypeScript frontend",
    "Tailwind design tokens",
    "Local-first project workflow",
  ];
</script>

<main class="min-h-screen bg-[var(--background)] text-[var(--text)]">
  <section
    class="mx-auto flex min-h-screen w-full max-w-5xl flex-col px-6 py-8"
  >
    <header
      class="flex h-10 items-center justify-between border-b border-[var(--border-muted)]"
    >
      <div class="flex items-center gap-3">
        <div
          class="grid size-7 place-items-center rounded-md border border-[var(--border)] bg-[var(--surface)] text-[var(--accent)]"
        >
          L
        </div>
        <div>
          <p class="text-sm font-medium leading-none">Larik</p>
          <p class="mt-1 text-xs text-[var(--text-subtle)]">
            Lightweight project-aware code editor
          </p>
        </div>
      </div>
      <span
        class="rounded-md border border-[var(--border)] px-2 py-1 text-xs text-[var(--text-muted)]"
      >
        Setup 0.1
      </span>
    </header>

    <div class="grid flex-1 place-items-center py-12">
      <div class="w-full max-w-2xl">
        <p
          class="mb-3 text-xs font-medium uppercase tracking-normal text-[var(--accent)]"
        >
          Project setup ready
        </p>
        <h1 class="text-3xl font-semibold leading-tight">
          Editor ringan yang ngerti project.
        </h1>
        <p class="mt-4 max-w-xl text-sm leading-6 text-[var(--text-muted)]">
          Larik disiapkan sebagai workspace coding lokal untuk membuka project,
          edit file, jalankan command, inspect output, perbaiki masalah, dan
          siap deploy tanpa extension bloat.
        </p>

        <div
          class="mt-8 rounded-lg border border-[var(--border)] bg-[var(--surface)]"
        >
          <div class="border-b border-[var(--border-muted)] px-4 py-3">
            <p class="text-sm font-medium">Foundation</p>
          </div>
          <ul class="divide-y divide-[var(--border-muted)]">
            {#each setupItems as item}
              <li
                class="flex items-center gap-3 px-4 py-3 text-sm text-[var(--text-muted)]"
              >
                <span class="size-2 rounded-full bg-[var(--accent)]"></span>
                {item}
              </li>
            {/each}
          </ul>
        </div>
      </div>
    </div>

    <footer
      class="flex h-8 items-center justify-between border-t border-[var(--border-muted)] text-xs text-[var(--text-subtle)]"
    >
      <span>Local-first</span>
      <span>AI optional</span>
    </footer>
  </section>
</main>
