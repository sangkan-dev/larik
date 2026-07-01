<script lang="ts">
  import { tick } from "svelte";
  import { fuzzySearch, type FuzzySearchItem } from "$lib/search/fuzzy";

  export let open = false;
  export let title: string;
  export let placeholder: string;
  export let items: FuzzySearchItem[] = [];
  export let onClose: () => void;
  export let onSelect: (item: FuzzySearchItem) => void;

  let query = "";
  let activeIndex = 0;
  let inputElement: HTMLInputElement;

  $: results = fuzzySearch(items, query).slice(0, 80);

  $: if (activeIndex >= results.length) {
    activeIndex = Math.max(results.length - 1, 0);
  }

  $: if (open) {
    focusInput();
  } else {
    query = "";
    activeIndex = 0;
  }

  async function focusInput() {
    await tick();
    inputElement?.focus();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      event.preventDefault();
      onClose();
      return;
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      activeIndex = Math.min(activeIndex + 1, Math.max(results.length - 1, 0));
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      activeIndex = Math.max(activeIndex - 1, 0);
      return;
    }

    if (event.key === "Enter") {
      event.preventDefault();
      const selected = results[activeIndex];

      if (selected) {
        onSelect(selected);
        onClose();
      }
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 bg-black/20"
    role="presentation"
    onclick={handleBackdropClick}
  >
    <div
      class="mx-auto mt-16 w-[min(720px,calc(100vw-2rem))] overflow-hidden rounded-lg border border-[var(--border)] bg-[var(--surface)] shadow-2xl"
      role="dialog"
      aria-modal="true"
      aria-label={title}
    >
      <div class="border-b border-[var(--border-muted)] p-3">
        <p class="mb-2 text-xs font-semibold text-[var(--text-muted)]">
          {title}
        </p>
        <input
          bind:this={inputElement}
          bind:value={query}
          class="h-9 w-full rounded-md border border-[var(--border-muted)] bg-[var(--background)] px-3 text-sm text-[var(--text)] outline-none focus:border-[var(--accent)]"
          {placeholder}
          onkeydown={handleKeydown}
        />
      </div>

      <div class="max-h-[min(520px,60vh)] overflow-auto p-1">
        {#if results.length === 0}
          <div class="px-3 py-8 text-center text-sm text-[var(--text-subtle)]">
            No matches
          </div>
        {:else}
          {#each results as item, index (item.id)}
            <button
              type="button"
              class={`flex min-h-11 w-full items-center gap-3 rounded-md px-3 py-2 text-left ${activeIndex === index ? "bg-[var(--accent-muted)] text-[var(--text)]" : "text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"}`}
              onmouseenter={() => {
                activeIndex = index;
              }}
              onclick={() => {
                onSelect(item);
                onClose();
              }}
            >
              <div class="min-w-0 flex-1">
                <p class="truncate text-sm">{item.title}</p>
                {#if item.subtitle}
                  <p class="mt-0.5 truncate text-xs text-[var(--text-subtle)]">
                    {item.subtitle}
                  </p>
                {/if}
              </div>
              {#if item.shortcut}
                <span
                  class="shrink-0 rounded border border-[var(--border-muted)] px-2 py-1 text-xs text-[var(--text-subtle)]"
                >
                  {item.shortcut}
                </span>
              {/if}
            </button>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}
