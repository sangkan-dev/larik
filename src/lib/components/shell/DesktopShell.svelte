<script lang="ts">
  import { onMount } from "svelte";
  import FileTree from "$lib/components/workspace/FileTree.svelte";
  import {
    activeDocument,
    activeFile,
    activeTab,
    closeTab,
    createEntry,
    deleteEntry,
    diskChange,
    expandedFolders,
    fileTree,
    initializeWorkspace,
    keepLocalDocument,
    openFile,
    openWorkspace,
    panelState,
    recentWorkspaces,
    registerWorkspaceWatcher,
    reloadDiskChangedDocument,
    reloadFileTree,
    renameEntry,
    saveActiveDocument,
    setActiveView,
    setBottomHeight,
    setBottomView,
    setSidebarWidth,
    tabs,
    toggleBottomPanel,
    toggleFolder,
    toggleSidebar,
    updateActiveDocument,
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

  onMount(() => {
    initializeWorkspace();
    registerWorkspaceWatcher();
  });

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

  function createFromPrompt(parentPath: string, type: "file" | "folder") {
    const name = window.prompt(
      type === "file" ? "New file name" : "New folder name",
    );

    if (!name) {
      return;
    }

    createEntry(parentPath, name, type);
  }

  function renameFromPrompt(path: string, currentName: string) {
    const nextName = window.prompt("Rename", currentName);

    if (!nextName || nextName === currentName) {
      return;
    }

    renameEntry(path, nextName);
  }

  function deleteWithConfirmation(path: string) {
    if (window.confirm(`Delete ${path}?`)) {
      deleteEntry(path);
    }
  }

  function closeTabWithConfirmation(tabId: string) {
    const tab = $tabs.find((item) => item.id === tabId);

    if (!tab) {
      return;
    }

    if (tab.dirty && !window.confirm(`Close ${tab.title} without saving?`)) {
      return;
    }

    if (tab.dirty) {
      tabs.update((items) =>
        items.map((item) =>
          item.id === tab.id ? { ...item, dirty: false } : item,
        ),
      );
    }

    closeTab(tabId);
  }
</script>

<main
  class="flex h-screen min-h-0 flex-col bg-[var(--background)] text-[var(--text)]"
>
  <header
    class="flex h-10 shrink-0 items-center justify-between border-b border-[var(--border-muted)] bg-[var(--surface)] px-3"
  >
    <div class="flex min-w-0 items-center gap-3">
      <div
        class="grid size-6 place-items-center rounded-md border border-[var(--border)] bg-[var(--surface-muted)] text-xs font-semibold text-[var(--accent)]"
      >
        L
      </div>
      <div class="flex min-w-0 items-center gap-2 text-sm">
        <span class="font-medium">Larik</span>
        <span class="text-[var(--text-subtle)]">/</span>
        <span class="truncate text-[var(--text-muted)]">
          {$workspace.name ?? "No Folder Open"}
        </span>
      </div>
    </div>

    <button
      type="button"
      class="hidden h-7 w-full max-w-md items-center rounded-md border border-[var(--border-muted)] bg-[var(--background)] px-3 text-left text-xs text-[var(--text-subtle)] hover:border-[var(--accent)] md:flex"
      onclick={() => openWorkspace()}
    >
      Open Folder
    </button>

    <div class="flex items-center gap-2 text-xs text-[var(--text-subtle)]">
      <span>{$workspace.rootPath ? "Workspace" : "Local"}</span>
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

        <div class="h-[calc(100%-2.25rem)] overflow-auto p-3">
          {#if $panelState.activeView === "explorer"}
            {#if !$workspace.rootPath}
              <p class="mb-3 text-xs text-[var(--text-subtle)]">
                No workspace is open yet.
              </p>
              <button
                type="button"
                class="h-8 w-full rounded-md border border-[var(--border)] bg-[var(--surface-muted)] px-3 text-left text-sm text-[var(--text)] hover:border-[var(--accent)]"
                onclick={() => openWorkspace()}
              >
                Open Folder
              </button>
              <div class="mt-4 border-t border-[var(--border-muted)] pt-3">
                <p class="mb-2 text-xs font-medium text-[var(--text-muted)]">
                  Recent
                </p>
                {#if $recentWorkspaces.length > 0}
                  <div class="space-y-1">
                    {#each $recentWorkspaces as recent}
                      <button
                        type="button"
                        class="w-full truncate rounded px-2 py-1.5 text-left text-xs text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                        title={recent}
                        onclick={() => openWorkspace(recent)}
                      >
                        {recent}
                      </button>
                    {/each}
                  </div>
                {:else}
                  <p class="text-xs text-[var(--text-subtle)]">
                    Recent workspaces will appear here.
                  </p>
                {/if}
              </div>
            {:else}
              <div class="mb-3 flex items-center justify-between gap-2">
                <button
                  type="button"
                  class="min-w-0 flex-1 truncate text-left text-xs font-medium text-[var(--text-muted)]"
                  title={$workspace.rootPath}
                  onclick={() => toggleFolder($workspace.rootPath ?? "")}
                >
                  {$workspace.name}
                </button>
                <button
                  type="button"
                  class="grid size-7 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                  title="Refresh explorer"
                  aria-label="Refresh explorer"
                  onclick={reloadFileTree}
                >
                  ↻
                </button>
                <button
                  type="button"
                  class="grid size-7 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                  title="New file"
                  aria-label="New file"
                  onclick={() =>
                    createFromPrompt($workspace.rootPath ?? "", "file")}
                >
                  +
                </button>
                <button
                  type="button"
                  class="grid size-7 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                  title="New folder"
                  aria-label="New folder"
                  onclick={() =>
                    createFromPrompt($workspace.rootPath ?? "", "folder")}
                >
                  □
                </button>
              </div>

              {#if $fileTree.loading}
                <p class="text-xs text-[var(--text-subtle)]">Loading tree...</p>
              {:else if $fileTree.error}
                <p class="text-xs text-[var(--danger)]">{$fileTree.error}</p>
              {:else}
                <FileTree
                  entries={$fileTree.entries}
                  expandedFolders={$expandedFolders}
                  onCreateFile={(path) => createFromPrompt(path, "file")}
                  onCreateFolder={(path) => createFromPrompt(path, "folder")}
                  onDelete={deleteWithConfirmation}
                  onOpenFile={openFile}
                  onRename={renameFromPrompt}
                  onToggleFolder={toggleFolder}
                />
              {/if}
            {/if}
          {:else if $panelState.activeView === "project"}
            <p class="text-xs text-[var(--text-subtle)]">
              Project actions will be detected after workspace scanning.
            </p>
          {:else if $panelState.activeView === "git"}
            <p class="text-xs text-[var(--text-subtle)]">
              Open a Git workspace to see branch and changed files.
            </p>
          {:else}
            <p class="text-xs text-[var(--text-subtle)]">
              Workspace search will be available after indexing.
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
        class="flex h-9 shrink-0 items-end overflow-x-auto border-b border-[var(--border-muted)] bg-[var(--surface)] pl-2"
      >
        {#each $tabs as tab}
          <button
            type="button"
            class={`group flex h-8 min-w-32 max-w-56 items-center gap-2 rounded-t-md border-x border-t px-3 text-left text-xs ${$activeTab?.id === tab.id ? "border-[var(--border)] bg-[var(--background)] text-[var(--text)]" : "border-transparent text-[var(--text-muted)] hover:bg-[var(--surface-muted)]"}`}
            onclick={() => activeFile.set(tab.id)}
          >
            <span class="truncate">{tab.title}</span>
            {#if tab.dirty}
              <span class="size-1.5 rounded-full bg-[var(--warning)]"></span>
            {/if}
            {#if tab.id !== "welcome"}
              <span
                role="button"
                tabindex="0"
                class="ml-auto hidden rounded px-1 text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)] group-hover:inline"
                onclick={(event) => {
                  event.stopPropagation();
                  closeTabWithConfirmation(tab.id);
                }}
                onkeydown={(event) => {
                  if (event.key === "Enter" || event.key === " ") {
                    event.preventDefault();
                    closeTabWithConfirmation(tab.id);
                  }
                }}
              >
                ×
              </span>
            {/if}
          </button>
        {/each}
      </div>

      <div class="relative grid min-h-0 flex-1 bg-[var(--background)]">
        {#if $diskChange}
          <div
            class="absolute right-4 top-4 z-10 flex max-w-lg items-center gap-3 rounded-md border border-[var(--warning)] bg-[var(--surface)] px-3 py-2 text-xs shadow"
          >
            <span class="truncate text-[var(--text-muted)]">
              File changed on disk: {$diskChange.path}
            </span>
            <button
              type="button"
              class="rounded bg-[var(--surface-muted)] px-2 py-1 text-[var(--text)]"
              onclick={reloadDiskChangedDocument}
            >
              Reload
            </button>
            <button
              type="button"
              class="rounded px-2 py-1 text-[var(--text-muted)] hover:bg-[var(--surface-muted)]"
              onclick={keepLocalDocument}
            >
              Keep
            </button>
          </div>
        {/if}

        {#if $activeDocument}
          {#if $activeDocument.tooLarge}
            <div
              class="mx-auto flex w-full max-w-2xl flex-col justify-center px-8"
            >
              <p class="text-sm font-medium">File is too large to open.</p>
              <p class="mt-2 text-sm text-[var(--text-muted)]">
                Size: {$activeDocument.size} bytes. Larik currently guards files above
                2 MB.
              </p>
            </div>
          {:else if $activeDocument.binary}
            <div
              class="mx-auto flex w-full max-w-2xl flex-col justify-center px-8"
            >
              <p class="text-sm font-medium">
                Binary file preview is disabled.
              </p>
              <p class="mt-2 text-sm text-[var(--text-muted)]">
                Open this file with an external viewer for now.
              </p>
            </div>
          {:else}
            <div class="flex min-h-0 flex-col">
              <div
                class="flex h-9 shrink-0 items-center justify-between border-b border-[var(--border-muted)] px-3 text-xs text-[var(--text-subtle)]"
              >
                <span class="truncate">{$activeDocument.path}</span>
                <button
                  type="button"
                  class="rounded-md border border-[var(--border)] px-3 py-1 text-[var(--text-muted)] hover:border-[var(--accent)] hover:text-[var(--text)]"
                  onclick={saveActiveDocument}
                >
                  Save
                </button>
              </div>
              <textarea
                class="min-h-0 flex-1 resize-none bg-[var(--background)] p-4 font-mono text-sm leading-6 text-[var(--text)] outline-none"
                spellcheck="false"
                value={$activeDocument.content}
                oninput={(event) =>
                  updateActiveDocument(event.currentTarget.value)}></textarea>
            </div>
          {/if}
        {:else}
          <div
            class="mx-auto flex w-full max-w-3xl flex-col justify-center px-8"
          >
            <p
              class="mb-3 text-xs font-medium uppercase tracking-normal text-[var(--accent)]"
            >
              Workspace
            </p>
            <h1 class="text-2xl font-semibold">
              {$workspace.rootPath
                ? "Select a file from Explorer."
                : "Open a folder to start coding."}
            </h1>
            <p
              class="mt-4 max-w-2xl text-sm leading-6 text-[var(--text-muted)]"
            >
              Larik can now open local workspaces, render a filtered file tree,
              read and save text files, track dirty tabs, and react to file
              changes on disk.
            </p>
            {#if !$workspace.rootPath}
              <button
                type="button"
                class="mt-6 h-9 w-36 rounded-md border border-[var(--border)] bg-[var(--surface)] px-3 text-sm hover:border-[var(--accent)]"
                onclick={() => openWorkspace()}
              >
                Open Folder
              </button>
            {/if}
          </div>
        {/if}
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
              <p>Workspace events and command output will appear here.</p>
            {/if}
          </div>
        </section>
      {/if}
    </section>
  </div>

  <footer
    class="flex h-6 shrink-0 items-center justify-between border-t border-[var(--border-muted)] bg-[var(--surface)] px-3 text-xs text-[var(--text-subtle)]"
  >
    <div class="flex min-w-0 items-center gap-4">
      <span>{$workspace.rootPath ? "Workspace" : "No workspace"}</span>
      {#if $workspace.rootPath}
        <span class="truncate">{$workspace.rootPath}</span>
      {/if}
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
