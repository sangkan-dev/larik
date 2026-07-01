<script lang="ts">
  import {
    activeTab,
    panelState,
    setActiveView,
    setBottomHeight,
    setBottomView,
    setSidebarWidth,
    tabs,
    toggleBottomPanel,
    toggleSidebar,
    workspace,
    type BottomPanelView,
    type ShellView,
  } from "$lib/stores/shell";

  const activityItems: { id: ShellView; label: string; icon: string }[] = [
    { id: "explorer", label: "Explorer", icon: "E" },
    { id: "search", label: "Search", icon: "S" },
    { id: "git", label: "Git", icon: "G" },
    { id: "project", label: "Project", icon: "P" },
  ];

  const bottomTabs: { id: BottomPanelView; label: string }[] = [
    { id: "terminal", label: "Terminal" },
    { id: "problems", label: "Problems" },
    { id: "output", label: "Output" },
  ];

  const projectActions = [
    "Open Folder",
    "Quick Open",
    "Command Palette",
    "Toggle Terminal",
  ];

  function resizeSidebar(event: PointerEvent) {
    const startX = event.clientX;
    const startWidth = $panelState.sidebarWidth;

    function move(moveEvent: PointerEvent) {
      setSidebarWidth(startWidth + moveEvent.clientX - startX);
    }

    function stop() {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", stop);
    }

    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", stop);
  }

  function resizeBottomPanel(event: PointerEvent) {
    const startY = event.clientY;
    const startHeight = $panelState.bottomHeight;

    function move(moveEvent: PointerEvent) {
      setBottomHeight(startHeight + startY - moveEvent.clientY);
    }

    function stop() {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", stop);
    }

    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", stop);
  }
</script>

<main
  class="flex h-screen min-h-0 flex-col bg-[var(--background)] text-[var(--text)]"
>
  <header
    class="flex h-10 shrink-0 items-center justify-between border-b border-[var(--border-muted)] bg-[var(--surface)] px-3"
  >
    <div class="flex items-center gap-3">
      <div
        class="grid size-6 place-items-center rounded-md border border-[var(--border)] bg-[var(--surface-muted)] text-xs font-semibold text-[var(--accent)]"
      >
        L
      </div>
      <div class="flex items-center gap-2 text-sm">
        <span class="font-medium">Larik</span>
        <span class="text-[var(--text-subtle)]">/</span>
        <span class="text-[var(--text-muted)]"
          >{$workspace.name ?? "No Folder Open"}</span
        >
      </div>
    </div>

    <div
      class="flex w-full max-w-md items-center rounded-md border border-[var(--border-muted)] bg-[var(--background)] px-3 py-1 text-xs text-[var(--text-subtle)]"
    >
      Ctrl+P Quick Open · Ctrl+Shift+P Command Palette
    </div>

    <div class="flex items-center gap-2 text-xs text-[var(--text-subtle)]">
      <span>Local</span>
      <span class="size-1.5 rounded-full bg-[var(--success)]"></span>
    </div>
  </header>

  <div class="flex min-h-0 flex-1">
    <nav
      class="flex w-11 shrink-0 flex-col items-center border-r border-[var(--border-muted)] bg-[var(--surface)] py-2"
    >
      {#each activityItems as item}
        <button
          type="button"
          class={`mb-1 grid size-8 place-items-center rounded-md text-xs font-medium ${$panelState.activeView === item.id ? "bg-[var(--accent-muted)] text-[var(--accent-hover)]" : "text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"}`}
          aria-label={item.label}
          title={item.label}
          onclick={() => setActiveView(item.id)}
        >
          {item.icon}
        </button>
      {/each}

      <button
        type="button"
        class="mt-auto grid size-8 place-items-center rounded-md text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
        aria-label="Toggle sidebar"
        title="Toggle sidebar"
        onclick={toggleSidebar}
      >
        {$panelState.sidebarVisible ? "‹" : "›"}
      </button>
    </nav>

    {#if $panelState.sidebarVisible}
      <aside
        class="min-w-0 shrink-0 border-r border-[var(--border-muted)] bg-[var(--surface)]"
        style={`width: ${$panelState.sidebarWidth}px`}
      >
        <div
          class="flex h-9 items-center justify-between border-b border-[var(--border-muted)] px-3"
        >
          <p
            class="text-xs font-semibold uppercase tracking-normal text-[var(--text-muted)]"
          >
            {$panelState.activeView}
          </p>
          <button
            type="button"
            class="rounded px-2 py-1 text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
            aria-label="Collapse sidebar"
            onclick={toggleSidebar}
          >
            ×
          </button>
        </div>

        <div class="p-3">
          {#if $panelState.activeView === "explorer"}
            <p class="mb-3 text-xs text-[var(--text-subtle)]">
              No workspace is open yet.
            </p>
            <button
              class="h-8 w-full rounded-md border border-[var(--border)] bg-[var(--surface-muted)] px-3 text-left text-sm text-[var(--text)] hover:border-[var(--accent)]"
            >
              Open Folder
            </button>
            <div class="mt-4 border-t border-[var(--border-muted)] pt-3">
              <p class="mb-2 text-xs font-medium text-[var(--text-muted)]">
                Recent
              </p>
              <p class="text-xs text-[var(--text-subtle)]">
                Recent workspaces will appear here.
              </p>
            </div>
          {:else if $panelState.activeView === "project"}
            <p class="mb-3 text-xs text-[var(--text-subtle)]">
              Project actions will be detected after opening a folder.
            </p>
            <div class="space-y-1">
              {#each projectActions as action}
                <button
                  class="h-8 w-full rounded-md px-2 text-left text-sm text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                >
                  {action}
                </button>
              {/each}
            </div>
          {:else if $panelState.activeView === "git"}
            <p class="text-xs text-[var(--text-subtle)]">
              Open a Git workspace to see branch and changed files.
            </p>
          {:else}
            <p class="text-xs text-[var(--text-subtle)]">
              Workspace search will be available after file indexing.
            </p>
          {/if}
        </div>
      </aside>

      <button
        type="button"
        class="w-1 shrink-0 cursor-col-resize bg-transparent hover:bg-[var(--accent-muted)]"
        aria-label="Resize sidebar"
        onpointerdown={resizeSidebar}
      ></button>
    {/if}

    <section class="flex min-w-0 flex-1 flex-col">
      <div
        class="flex h-9 shrink-0 items-end border-b border-[var(--border-muted)] bg-[var(--surface)] pl-2"
      >
        {#each $tabs as tab}
          <button
            type="button"
            class={`flex h-8 min-w-32 max-w-52 items-center gap-2 rounded-t-md border-x border-t px-3 text-left text-xs ${$activeTab?.id === tab.id ? "border-[var(--border)] bg-[var(--background)] text-[var(--text)]" : "border-transparent text-[var(--text-muted)] hover:bg-[var(--surface-muted)]"}`}
          >
            <span class="truncate">{tab.title}</span>
            {#if tab.dirty}
              <span class="size-1.5 rounded-full bg-[var(--warning)]"></span>
            {/if}
          </button>
        {/each}
      </div>

      <div class="grid min-h-0 flex-1 bg-[var(--background)]">
        <div class="mx-auto flex w-full max-w-3xl flex-col justify-center px-8">
          <p
            class="mb-3 text-xs font-medium uppercase tracking-normal text-[var(--accent)]"
          >
            Desktop Shell MVP
          </p>
          <h1 class="text-2xl font-semibold">
            Code workspace foundation is ready.
          </h1>
          <p class="mt-4 max-w-2xl text-sm leading-6 text-[var(--text-muted)]">
            Shell layout, persistent UI state, editor tabs placeholder, sidebar
            panes, bottom panel, and status bar are now wired for the next
            workspace and file-system tasks.
          </p>
        </div>
      </div>

      {#if $panelState.bottomVisible}
        <button
          type="button"
          class="h-1 shrink-0 cursor-row-resize bg-transparent hover:bg-[var(--accent-muted)]"
          aria-label="Resize bottom panel"
          onpointerdown={resizeBottomPanel}
        ></button>

        <section
          class="shrink-0 border-t border-[var(--border-muted)] bg-[var(--surface)]"
          style={`height: ${$panelState.bottomHeight}px`}
        >
          <div
            class="flex h-9 items-center justify-between border-b border-[var(--border-muted)] px-2"
          >
            <div class="flex items-center gap-1">
              {#each bottomTabs as tab}
                <button
                  type="button"
                  class={`h-7 rounded-md px-3 text-xs ${$panelState.bottomView === tab.id ? "bg-[var(--surface-muted)] text-[var(--text)]" : "text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"}`}
                  onclick={() => setBottomView(tab.id)}
                >
                  {tab.label}
                </button>
              {/each}
            </div>

            <button
              type="button"
              class="rounded px-2 py-1 text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
              aria-label="Collapse bottom panel"
              onclick={toggleBottomPanel}
            >
              ×
            </button>
          </div>

          <div
            class="h-[calc(100%-2.25rem)] p-3 font-mono text-xs text-[var(--text-muted)]"
          >
            {#if $panelState.bottomView === "terminal"}
              <p>$ terminal backend will attach here</p>
            {:else if $panelState.bottomView === "problems"}
              <p>No problems detected.</p>
            {:else}
              <p>Output logs will appear here.</p>
            {/if}
          </div>
        </section>
      {/if}
    </section>
  </div>

  <footer
    class="flex h-6 shrink-0 items-center justify-between border-t border-[var(--border-muted)] bg-[var(--surface)] px-3 text-xs text-[var(--text-subtle)]"
  >
    <div class="flex items-center gap-4">
      <span>{$workspace.rootPath ? "Workspace" : "No workspace"}</span>
      <span>main</span>
      <span>LSP idle</span>
    </div>
    <div class="flex items-center gap-4">
      <button
        class="hover:text-[var(--text)]"
        type="button"
        onclick={toggleBottomPanel}
      >
        {$panelState.bottomVisible ? "Hide Panel" : "Show Panel"}
      </button>
      <span>UTF-8</span>
      <span>Ln 1, Col 1</span>
    </div>
  </footer>
</main>
