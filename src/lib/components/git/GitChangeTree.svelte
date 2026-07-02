<script lang="ts">
  import type { GitChangedFile } from "$lib/services/git";

  type GitTreeRow =
    | {
        type: "dir";
        key: string;
        name: string;
        depth: number;
      }
    | {
        type: "file";
        key: string;
        name: string;
        depth: number;
        file: GitChangedFile;
      };

  export let files: GitChangedFile[] = [];
  export let action: "stage" | "unstage" | null = null;
  export let inspectStaged = false;
  export let selectedKey: string | null = null;
  export let onInspect: (file: GitChangedFile, staged: boolean) => void;
  export let onOpen: (file: GitChangedFile) => void;
  export let onStage: (file: GitChangedFile) => void;
  export let onUnstage: (file: GitChangedFile) => void;
  export let disabled = false;

  let expandedDirs = new Set<string>();

  $: rows = buildRows(files, inspectStaged);
  $: for (const row of rows) {
    if (row.type === "dir") {
      expandedDirs.add(row.key);
    }
  }

  function buildRows(items: GitChangedFile[], staged: boolean) {
    const rows: GitTreeRow[] = [];
    const seenDirs = new Set<string>();

    for (const file of [...items].sort((left, right) =>
      left.path.localeCompare(right.path),
    )) {
      const parts = file.path.split("/").filter(Boolean);
      let prefix = "";

      for (const [index, part] of parts.slice(0, -1).entries()) {
        prefix = prefix ? `${prefix}/${part}` : part;
        if (!seenDirs.has(prefix)) {
          seenDirs.add(prefix);
          rows.push({
            type: "dir",
            key: prefix,
            name: part,
            depth: index,
          });
        }
      }

      rows.push({
        type: "file",
        key: `${file.path}:${staged ? "staged" : "worktree"}`,
        name: parts.at(-1) ?? file.path,
        depth: Math.max(parts.length - 1, 0),
        file,
      });
    }

    return rows;
  }

  function isVisible(row: GitTreeRow) {
    const parts = row.key.split("/");
    let prefix = "";

    for (const part of parts.slice(0, -1)) {
      prefix = prefix ? `${prefix}/${part}` : part;
      if (!expandedDirs.has(prefix)) {
        return false;
      }
    }

    return true;
  }

  function toggleDir(path: string) {
    const next = new Set(expandedDirs);
    if (next.has(path)) {
      next.delete(path);
    } else {
      next.add(path);
    }
    expandedDirs = next;
  }

  function actionLabel() {
    if (action === "stage") {
      return "+";
    }
    if (action === "unstage") {
      return "-";
    }
    return "";
  }

  function actionTitle() {
    if (action === "stage") {
      return "Stage";
    }
    if (action === "unstage") {
      return "Unstage";
    }
    return "";
  }
</script>

<div class="space-y-0.5">
  {#each rows as row (row.key)}
    {#if isVisible(row)}
      {#if row.type === "dir"}
        <button
          type="button"
          class="flex h-6 w-full items-center gap-1 rounded px-1.5 text-left text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
          style={`padding-left: ${row.depth * 12 + 6}px`}
          title={row.key}
          onclick={() => toggleDir(row.key)}
        >
          <span class="w-3">{expandedDirs.has(row.key) ? "▾" : "▸"}</span>
          <span class="truncate">{row.name}</span>
        </button>
      {:else}
        <div
          class={`group flex h-7 items-center gap-1 rounded px-1 ${selectedKey === row.key ? "bg-[var(--surface-muted)] text-[var(--text)]" : "text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"}`}
          style={`padding-left: ${row.depth * 12 + 6}px`}
        >
          <button
            type="button"
            class="flex min-w-0 flex-1 items-center gap-2 text-left text-xs"
            title={row.file.path}
            onclick={() => onInspect(row.file, inspectStaged)}
            ondblclick={() => onOpen(row.file)}
          >
            <span class="w-3 text-[var(--text-subtle)]">•</span>
            <span class="truncate">{row.name}</span>
            <span
              class="ml-auto shrink-0 text-[10px] text-[var(--text-subtle)]"
            >
              {row.file.kind}
            </span>
          </button>

          {#if action}
            <button
              type="button"
              class="hidden size-6 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--background)] hover:text-[var(--text)] disabled:cursor-not-allowed disabled:opacity-60 group-hover:grid"
              title={actionTitle()}
              aria-label={actionTitle()}
              {disabled}
              onclick={() =>
                action === "stage" ? onStage(row.file) : onUnstage(row.file)}
            >
              {actionLabel()}
            </button>
          {/if}
        </div>
      {/if}
    {/if}
  {/each}
</div>
