<script lang="ts">
  import { get } from "svelte/store";
  import XtermTerminal from "$lib/components/terminal/XtermTerminal.svelte";
  import {
    activeTerminalId,
    closeTerminal,
    createTerminal,
    setActiveTerminal,
    terminalSessions,
  } from "$lib/stores/terminal";

  export let cwd: string | null = null;

  let terminalRef: XtermTerminal | null = null;
  let focused = false;

  $: activeSession =
    $terminalSessions.find((session) => session.id === $activeTerminalId) ??
    null;

  async function createShellTerminal() {
    await createTerminal({
      cwd,
      label: "shell",
    });
  }

  async function runCommandFromPrompt() {
    const command = window.prompt("Run command in workspace");

    if (!command) {
      return;
    }

    await createTerminal({
      cwd,
      command,
      label: command,
    });
  }

  async function closeActiveTerminal() {
    const sessionId = get(activeTerminalId);

    if (sessionId) {
      await closeTerminal(sessionId);
    }
  }
</script>

<div class="flex h-full min-h-0 flex-col">
  <div
    class="flex h-8 shrink-0 items-center justify-between border-b border-[var(--border-muted)] px-2"
  >
    <div class="flex min-w-0 items-center gap-1">
      {#if $terminalSessions.length === 0}
        <span class="px-2 text-xs text-[var(--text-subtle)]">
          No terminal session
        </span>
      {:else}
        {#each $terminalSessions as session}
          <button
            type="button"
            class={`flex max-w-40 items-center gap-2 rounded px-2 py-1 text-xs ${session.id === $activeTerminalId ? "bg-[var(--surface-muted)] text-[var(--text)]" : "text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"}`}
            title={session.cwd ?? session.label}
            onclick={() => setActiveTerminal(session.id)}
          >
            <span class="truncate">{session.label}</span>
            {#if session.exited}
              <span class="text-[var(--warning)]">exit</span>
            {/if}
          </button>
        {/each}
      {/if}
    </div>

    <div class="flex items-center gap-1">
      <span class="px-2 text-xs text-[var(--text-subtle)]">
        {focused ? "focused" : "idle"}
      </span>
      <button
        type="button"
        class="rounded px-2 py-1 text-xs text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
        onclick={createShellTerminal}
      >
        New
      </button>
      <button
        type="button"
        class="rounded px-2 py-1 text-xs text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
        onclick={runCommandFromPrompt}
      >
        Run
      </button>
      <button
        type="button"
        class="rounded px-2 py-1 text-xs text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
        onclick={() => terminalRef?.copySelection()}
      >
        Copy
      </button>
      <button
        type="button"
        class="rounded px-2 py-1 text-xs text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
        onclick={() => terminalRef?.pasteClipboard()}
      >
        Paste
      </button>
      <button
        type="button"
        class="rounded px-2 py-1 text-xs text-[var(--danger)] hover:bg-[var(--surface-muted)]"
        onclick={closeActiveTerminal}
      >
        Kill
      </button>
    </div>
  </div>

  <div
    class="min-h-0 flex-1 bg-[#111418]"
    onfocusin={() => {
      focused = true;
    }}
    onfocusout={() => {
      focused = false;
    }}
  >
    {#if activeSession}
      <XtermTerminal
        bind:this={terminalRef}
        sessionId={activeSession.id}
        output={activeSession.output}
      />
    {:else}
      <div
        class="flex h-full items-center justify-center text-xs text-[var(--text-subtle)]"
      >
        <button
          type="button"
          class="rounded-md border border-[var(--border)] px-3 py-2 text-[var(--text-muted)] hover:border-[var(--accent)] hover:text-[var(--text)]"
          onclick={createShellTerminal}
        >
          New Terminal
        </button>
      </div>
    {/if}
  </div>
</div>
