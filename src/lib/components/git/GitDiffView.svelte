<script lang="ts">
  import type { GitChangedFile, GitDiffResponse } from "$lib/services/git";

  export let file: GitChangedFile;
  export let diff: GitDiffResponse;
  export let loading = false;
  export let onOpenFile: (file: GitChangedFile) => void;

  $: lines = diff.content ? diff.content.split("\n") : [];

  function lineClass(line: string) {
    if (line.startsWith("+") && !line.startsWith("+++")) {
      return "bg-[color-mix(in_srgb,var(--success)_12%,transparent)] text-[var(--success)]";
    }
    if (line.startsWith("-") && !line.startsWith("---")) {
      return "bg-[color-mix(in_srgb,var(--danger)_12%,transparent)] text-[var(--danger)]";
    }
    if (line.startsWith("@@")) {
      return "text-[var(--info)]";
    }
    if (line.startsWith("diff --git") || line.startsWith("index ")) {
      return "text-[var(--text-subtle)]";
    }

    return "text-[var(--text-muted)]";
  }
</script>

<div class="flex min-h-0 flex-1 flex-col">
  <div
    class="flex h-10 shrink-0 items-center justify-between border-b border-[var(--border-muted)] px-3 text-xs"
  >
    <div class="min-w-0">
      <p class="truncate text-[var(--text)]">{file.path}</p>
      <p class="truncate text-[var(--text-subtle)]">
        {diff.staged ? "Staged changes" : "Working tree changes"}
      </p>
    </div>
    <button
      type="button"
      class="h-7 rounded-md border border-[var(--border)] px-3 text-[var(--text-muted)] hover:border-[var(--accent)] hover:text-[var(--text)] disabled:cursor-not-allowed disabled:opacity-60"
      disabled={file.kind === "deleted"}
      onclick={() => onOpenFile(file)}
    >
      Open file
    </button>
  </div>

  <div class="min-h-0 flex-1 overflow-auto bg-[var(--background)]">
    {#if loading}
      <p class="p-4 text-xs text-[var(--text-subtle)]">Loading diff...</p>
    {:else if lines.length > 0}
      <pre
        class="min-w-max p-4 font-mono text-xs leading-5">{#each lines as line, index}
          <span class={`block px-2 ${lineClass(line)}`}
            ><span
              class="mr-4 inline-block w-10 select-none text-right text-[var(--text-subtle)]"
              >{index + 1}</span
            >{line || " "}</span
          >{/each}</pre>
    {:else if file.untracked}
      <p class="p-4 text-xs text-[var(--text-subtle)]">
        This file is new. Stage it first to review it in the Git diff.
      </p>
    {:else}
      <p class="p-4 text-xs text-[var(--text-subtle)]">
        No diff content for this file state.
      </p>
    {/if}
  </div>
</div>
