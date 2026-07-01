<script lang="ts">
  import type { FileTreeEntry } from "$lib/services/workspace";

  export let entries: FileTreeEntry[] = [];
  export let expandedFolders: string[] = [];
  export let onCreateFile: (parentPath: string) => void;
  export let onCreateFolder: (parentPath: string) => void;
  export let onDelete: (path: string) => void;
  export let onOpenFile: (path: string) => void;
  export let onRename: (path: string, currentName: string) => void;
  export let onToggleFolder: (path: string) => void;

  function isExpanded(path: string) {
    return expandedFolders.includes(path);
  }
</script>

<ul class="space-y-0.5">
  {#each entries as entry (entry.path)}
    <li>
      <div class="group flex h-7 min-w-0 items-center gap-1">
        <button
          type="button"
          class="flex min-w-0 flex-1 items-center gap-2 rounded px-1.5 py-1 text-left text-xs text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
          title={entry.path}
          onclick={() =>
            entry.isDir ? onToggleFolder(entry.path) : onOpenFile(entry.path)}
        >
          <span class="w-3 text-[var(--text-subtle)]">
            {#if entry.isDir}
              {isExpanded(entry.path) ? "▾" : "▸"}
            {:else}
              •
            {/if}
          </span>
          <span class="truncate">{entry.name}</span>
        </button>

        {#if entry.isDir}
          <button
            type="button"
            class="hidden size-6 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)] group-hover:grid"
            title="New file"
            aria-label="New file"
            onclick={() => onCreateFile(entry.path)}
          >
            +
          </button>
          <button
            type="button"
            class="hidden size-6 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)] group-hover:grid"
            title="New folder"
            aria-label="New folder"
            onclick={() => onCreateFolder(entry.path)}
          >
            □
          </button>
        {/if}

        <button
          type="button"
          class="hidden size-6 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)] group-hover:grid"
          title="Rename"
          aria-label="Rename"
          onclick={() => onRename(entry.path, entry.name)}
        >
          r
        </button>
        <button
          type="button"
          class="hidden size-6 place-items-center rounded text-xs text-[var(--danger)] hover:bg-[var(--surface-muted)] group-hover:grid"
          title="Delete"
          aria-label="Delete"
          onclick={() => onDelete(entry.path)}
        >
          ×
        </button>
      </div>

      {#if entry.isDir && isExpanded(entry.path) && entry.children}
        <div class="ml-4 border-l border-[var(--border-muted)] pl-2">
          <svelte:self
            entries={entry.children}
            {expandedFolders}
            {onCreateFile}
            {onCreateFolder}
            {onDelete}
            {onOpenFile}
            {onRename}
            {onToggleFolder}
          />
        </div>
      {/if}
    </li>
  {/each}
</ul>
