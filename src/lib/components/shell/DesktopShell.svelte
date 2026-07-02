<script lang="ts">
  import { onMount } from "svelte";
  import CommandLauncher from "$lib/components/command/CommandLauncher.svelte";
  import MonacoEditor from "$lib/components/editor/MonacoEditor.svelte";
  import TerminalPanel from "$lib/components/terminal/TerminalPanel.svelte";
  import FileTree from "$lib/components/workspace/FileTree.svelte";
  import {
    defaultKeybindings,
    keybindingFromEvent,
    type KeybindingCommandId,
  } from "$lib/commands/keybindings";
  import {
    baseCommandMetadata,
    createCommandRegistry,
  } from "$lib/commands/registry";
  import type { MonacoEditorController } from "$lib/editor/types";
  import type { FuzzySearchItem } from "$lib/search/fuzzy";
  import type { GitChangedFile } from "$lib/services/git";
  import type { ProjectAction } from "$lib/services/projectDetector";
  import { gitState, refreshGitStatus, selectGitFile } from "$lib/stores/git";
  import {
    clearProjectDetection,
    markProjectActionFailed,
    markProjectActionRunning,
    projectActionRuns,
    projectDetection,
    scanProject,
  } from "$lib/stores/projectDetector";
  import { createTerminal, registerTerminalEvents } from "$lib/stores/terminal";
  import { indexWorkspaceFiles } from "$lib/workspace/fileIndex";
  import {
    activeDocument,
    activeFile,
    activeTab,
    closeAllTabs,
    closeOtherTabs,
    closeTab,
    createEntry,
    deleteEntry,
    diskChange,
    editorPreferences,
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
    reopenRecentlyClosedTab,
    saveActiveDocument,
    saveAllDocuments,
    setActiveView,
    setBottomHeight,
    setBottomView,
    setSidebarWidth,
    tabs,
    toggleBottomPanel,
    toggleEditorMinimap,
    toggleEditorWordWrap,
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

  let editorController: MonacoEditorController | null = null;
  let commandPaletteOpen = false;
  let quickOpenOpen = false;
  let detectedRootPath: string | null = null;

  $: commandRegistry = createCommandRegistry(
    baseCommandMetadata.map((command) => ({
      ...command,
      handler: () => executeCommand(command.id),
    })),
  );
  $: commandPaletteItems = commandRegistry.all().map((command) => ({
    id: command.id,
    title: command.title,
    subtitle: command.category,
    shortcut: command.shortcut,
  }));
  $: quickOpenItems = indexWorkspaceFiles($fileTree.entries);
  $: if ($workspace.rootPath && $workspace.rootPath !== detectedRootPath) {
    detectedRootPath = $workspace.rootPath;
    scanProject($workspace.rootPath);
  } else if (!$workspace.rootPath && detectedRootPath) {
    detectedRootPath = null;
    clearProjectDetection();
  }

  onMount(() => {
    initializeWorkspace();
    registerWorkspaceWatcher();
    registerTerminalEvents();
    window.addEventListener("keydown", handleKeydown);

    return () => window.removeEventListener("keydown", handleKeydown);
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

    if (closeTab(tabId)) {
      editorController?.disposeModel(tab.path);
    }
  }

  function closeOtherTabsWithConfirmation(tabId: string) {
    const dirtyTabs = $tabs.filter((tab) => tab.id !== tabId && tab.dirty);

    if (
      dirtyTabs.length > 0 &&
      !window.confirm("Close other tabs with unsaved changes?")
    ) {
      return;
    }

    for (const tab of $tabs) {
      if (tab.id !== tabId && tab.id !== "welcome") {
        editorController?.disposeModel(tab.path);
      }
    }

    if (dirtyTabs.length > 0) {
      tabs.update((items) =>
        items.map((item) =>
          item.id !== tabId ? { ...item, dirty: false } : item,
        ),
      );
    }

    closeOtherTabs(tabId);
  }

  function closeAllTabsWithConfirmation() {
    const dirtyTabs = $tabs.filter((tab) => tab.dirty);

    if (
      dirtyTabs.length > 0 &&
      !window.confirm("Close all tabs with unsaved changes?")
    ) {
      return;
    }

    for (const tab of $tabs) {
      if (tab.id !== "welcome") {
        editorController?.disposeModel(tab.path);
      }
    }

    if (dirtyTabs.length > 0) {
      tabs.update((items) => items.map((item) => ({ ...item, dirty: false })));
    }

    closeAllTabs();
  }

  function executeCommand(command: KeybindingCommandId) {
    if (command === "workspace.openFolder") openWorkspace();
    if (command === "editor.save") saveActiveDocument();
    if (command === "editor.saveAll") saveAllDocuments();
    if (command === "file.quickOpen") quickOpenOpen = true;
    if (command === "commandPalette.open") {
      commandPaletteOpen = true;
    }
    if (command === "terminal.toggle") {
      setBottomView("terminal");
      toggleBottomPanel();
    }
    if (command === "view.toggleSidebar") toggleSidebar();
    if (command === "editor.closeTab" && $activeTab) {
      closeTabWithConfirmation($activeTab.id);
    }
    if (command === "editor.find") editorController?.find();
    if (command === "editor.replace") editorController?.replace();
    if (command === "editor.goToLine") editorController?.goToLine();
    if (command === "editor.toggleMinimap") {
      toggleEditorMinimap();
      editorController?.toggleMinimap();
    }
    if (command === "editor.toggleWordWrap") {
      toggleEditorWordWrap();
      editorController?.toggleWordWrap();
    }
    if (command === "editor.formatDocument") editorController?.formatDocument();
  }

  function handleKeydown(event: KeyboardEvent) {
    const binding = defaultKeybindings.find(
      (item) => item.key === keybindingFromEvent(event),
    );

    if (!binding) {
      return;
    }

    event.preventDefault();
    commandRegistry.execute(binding.command);
  }

  function selectCommand(item: FuzzySearchItem) {
    commandRegistry.execute(item.id as KeybindingCommandId);
  }

  function selectQuickOpenFile(item: FuzzySearchItem) {
    openFile(item.id);
  }

  function refreshGitPanel() {
    if ($workspace.rootPath) {
      refreshGitStatus($workspace.rootPath);
    }
  }

  function gitIndicatorLabel() {
    if (!$workspace.rootPath) {
      return "No Git";
    }
    if ($gitState.loading) {
      return "Git scanning";
    }
    if ($gitState.error) {
      return "Git error";
    }
    if (!$gitState.status?.isRepo) {
      return "No Git repo";
    }

    const branch = $gitState.status.branch ?? "detached";
    const count = $gitState.status.changedFiles.length;
    const sync =
      $gitState.status.ahead > 0 || $gitState.status.behind > 0
        ? ` +${$gitState.status.ahead}/-${$gitState.status.behind}`
        : "";

    return `${branch}${sync} · ${count}`;
  }

  function gitFileBadge(file: GitChangedFile) {
    if (file.untracked) {
      return "untracked";
    }
    if (file.staged && file.unstaged) {
      return "staged + unstaged";
    }
    if (file.staged) {
      return "staged";
    }
    if (file.unstaged) {
      return "unstaged";
    }

    return file.kind;
  }

  function canOpenGitFile(file: GitChangedFile) {
    return file.kind !== "deleted";
  }

  function openGitFile(file: GitChangedFile) {
    if (canOpenGitFile(file)) {
      openFile(file.absolutePath);
    }
  }

  async function createWorkspaceTerminal() {
    setBottomView("terminal");
    if (!$panelState.bottomVisible) {
      toggleBottomPanel();
    }
    await createTerminal({
      cwd: $workspace.rootPath,
      label: "shell",
    });
  }

  async function runWorkspaceCommand() {
    const command = window.prompt("Run command in workspace");

    if (!command) {
      return;
    }

    setBottomView("terminal");
    if (!$panelState.bottomVisible) {
      toggleBottomPanel();
    }
    await createTerminal({
      cwd: $workspace.rootPath,
      command,
      label: command,
    });
  }

  async function runProjectAction(action: ProjectAction) {
    const existingRun = $projectActionRuns[action.id];
    if (action.destructive && existingRun?.status === "running") {
      window.alert(`${action.label} is already running.`);
      return;
    }

    if (
      action.destructive &&
      !window.confirm(`Run destructive action: ${action.command}?`)
    ) {
      return;
    }

    setBottomView("terminal");
    if (!$panelState.bottomVisible) {
      toggleBottomPanel();
    }
    try {
      const session = await createTerminal({
        cwd: action.cwd,
        command: action.command,
        label: action.label,
      });
      markProjectActionRunning(action, session.id);
    } catch (error) {
      markProjectActionFailed(
        action,
        error instanceof Error ? error.message : String(error),
      );
    }
  }

  function refreshProjectDetection() {
    if ($workspace.rootPath) {
      scanProject($workspace.rootPath);
    }
  }

  function projectDetail(project: { details: Record<string, unknown> }) {
    const framework = project.details.framework;
    const packageManager = project.details.packageManager;

    return [framework, packageManager]
      .filter((item): item is string => typeof item === "string")
      .join(" / ");
  }

  function projectScripts(project: { details: Record<string, unknown> }) {
    return Array.isArray(project.details.scripts)
      ? project.details.scripts.filter(
          (script): script is string => typeof script === "string",
        )
      : [];
  }

  function actionStatusLabel(action: ProjectAction) {
    const run = $projectActionRuns[action.id];

    if (!run) {
      return action.destructive ? "confirm" : "ready";
    }

    if (run.status === "running") {
      return "running";
    }

    if (run.status === "done") {
      return run.exitCode === null ? "done" : `done ${run.exitCode}`;
    }

    return run.exitCode === null ? "failed" : `failed ${run.exitCode}`;
  }

  function actionStatusClass(action: ProjectAction) {
    const run = $projectActionRuns[action.id];

    if (run?.status === "running") {
      return "text-[var(--info)]";
    }
    if (run?.status === "done") {
      return "text-[var(--success)]";
    }
    if (run?.status === "failed") {
      return "text-[var(--danger)]";
    }

    return action.destructive
      ? "text-[var(--warning)]"
      : "text-[var(--text-subtle)]";
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
            <div class="mb-3 flex items-center justify-between gap-2">
              <p class="text-xs font-medium text-[var(--text-muted)]">
                Detection
              </p>
              <button
                type="button"
                class="grid size-7 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                title="Refresh project detection"
                aria-label="Refresh project detection"
                onclick={refreshProjectDetection}
              >
                ↻
              </button>
            </div>

            {#if !$workspace.rootPath}
              <p class="mb-3 text-xs text-[var(--text-subtle)]">
                Open a workspace to detect project actions.
              </p>
            {:else if $projectDetection.loading}
              <p class="mb-3 text-xs text-[var(--text-subtle)]">
                Scanning workspace...
              </p>
            {:else if $projectDetection.error}
              <p class="mb-3 text-xs text-[var(--danger)]">
                {$projectDetection.error}
              </p>
            {:else if $projectDetection.result}
              {#if $projectDetection.result.detected.length > 0}
                <div class="mb-4 space-y-2">
                  {#each $projectDetection.result.detected as project}
                    <div
                      class="rounded-md border border-[var(--border-muted)] bg-[var(--background)] p-2"
                    >
                      <div class="flex items-center justify-between gap-2">
                        <p class="truncate text-sm text-[var(--text)]">
                          {project.name}
                        </p>
                        <span
                          class="shrink-0 text-xs text-[var(--text-subtle)]"
                        >
                          {Math.round(project.confidence * 100)}%
                        </span>
                      </div>
                      <p class="mt-1 truncate text-xs text-[var(--text-muted)]">
                        {project.kind}{projectDetail(project)
                          ? ` / ${projectDetail(project)}`
                          : ""}
                      </p>
                      {#if project.detectedFiles.length > 0}
                        <p
                          class="mt-1 truncate text-xs text-[var(--text-subtle)]"
                          title={project.detectedFiles.join(", ")}
                        >
                          {project.detectedFiles.join(", ")}
                        </p>
                      {/if}
                      {#if projectScripts(project).length > 0}
                        <div class="mt-2 flex flex-wrap gap-1">
                          {#each projectScripts(project) as script}
                            <span
                              class="rounded border border-[var(--border-muted)] px-1.5 py-0.5 text-xs text-[var(--text-subtle)]"
                            >
                              {script}
                            </span>
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {/each}
                </div>
              {:else}
                <p class="mb-3 text-xs text-[var(--text-subtle)]">
                  No project type detected yet.
                </p>
              {/if}

              {#if $projectDetection.result.warnings.length > 0}
                <div class="mb-4 space-y-1">
                  <p class="text-xs font-medium text-[var(--warning)]">
                    Warnings
                  </p>
                  {#each $projectDetection.result.warnings as warning}
                    <p class="text-xs text-[var(--text-muted)]">{warning}</p>
                  {/each}
                </div>
              {/if}

              {#if $projectDetection.result.env.hasEnv || $projectDetection.result.env.hasEnvExample}
                <div class="mb-4 space-y-1">
                  <p class="text-xs font-medium text-[var(--text-muted)]">
                    Environment
                  </p>
                  <p class="text-xs text-[var(--text-muted)]">
                    .env:
                    {$projectDetection.result.env.hasEnv ? "found" : "missing"}
                    / .env.example:
                    {$projectDetection.result.env.hasEnvExample
                      ? "found"
                      : "missing"}
                  </p>
                  {#if $projectDetection.result.env.missingKeys.length > 0}
                    <p
                      class="text-xs text-[var(--warning)]"
                      title={$projectDetection.result.env.missingKeys.join(
                        ", ",
                      )}
                    >
                      Missing keys:
                      {$projectDetection.result.env.missingKeys.join(", ")}
                    </p>
                  {/if}
                  {#if $projectDetection.result.env.emptyKeys.length > 0}
                    <p
                      class="text-xs text-[var(--warning)]"
                      title={$projectDetection.result.env.emptyKeys.join(", ")}
                    >
                      Empty keys:
                      {$projectDetection.result.env.emptyKeys.join(", ")}
                    </p>
                  {/if}
                </div>
              {/if}

              {#if $projectDetection.result.actions.length > 0}
                <div class="mb-4 space-y-1">
                  <p class="text-xs font-medium text-[var(--text-muted)]">
                    Actions
                  </p>
                  {#each $projectDetection.result.actions as action}
                    <button
                      type="button"
                      class="flex h-8 w-full items-center gap-2 rounded-md px-2 text-left text-sm text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)] disabled:cursor-not-allowed disabled:opacity-60"
                      title={action.command}
                      disabled={action.destructive &&
                        $projectActionRuns[action.id]?.status === "running"}
                      onclick={() => runProjectAction(action)}
                    >
                      <span class="min-w-0 flex-1 truncate">
                        {action.label}
                      </span>
                      <span
                        class={`shrink-0 text-xs ${actionStatusClass(action)}`}
                      >
                        {actionStatusLabel(action)}
                      </span>
                    </button>
                  {/each}
                </div>
              {/if}
            {/if}

            <div class="space-y-1">
              <button
                type="button"
                class="h-8 w-full rounded-md px-2 text-left text-sm text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                onclick={createWorkspaceTerminal}
              >
                New terminal
              </button>
              <button
                type="button"
                class="h-8 w-full rounded-md px-2 text-left text-sm text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                onclick={runWorkspaceCommand}
              >
                Run command
              </button>
            </div>
          {:else if $panelState.activeView === "git"}
            <div class="mb-3 flex items-center justify-between gap-2">
              <p class="text-xs font-medium text-[var(--text-muted)]">Git</p>
              <button
                type="button"
                class="grid size-7 place-items-center rounded text-xs text-[var(--text-subtle)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                title="Refresh Git status"
                aria-label="Refresh Git status"
                onclick={refreshGitPanel}
              >
                ↻
              </button>
            </div>

            {#if !$workspace.rootPath}
              <p class="text-xs text-[var(--text-subtle)]">
                Open a workspace to inspect Git status.
              </p>
            {:else if $gitState.loading}
              <p class="text-xs text-[var(--text-subtle)]">
                Reading Git status...
              </p>
            {:else if $gitState.error}
              <p class="text-xs text-[var(--danger)]">{$gitState.error}</p>
            {:else if !$gitState.status?.isRepo}
              <p class="text-xs text-[var(--text-subtle)]">
                This workspace is not a Git repository.
              </p>
            {:else}
              <div
                class="mb-4 rounded-md border border-[var(--border-muted)] bg-[var(--background)] p-2"
              >
                <div class="flex items-center justify-between gap-2">
                  <p class="truncate text-sm text-[var(--text)]">
                    {$gitState.status.branch ?? "detached"}
                  </p>
                  <span class="text-xs text-[var(--text-subtle)]">
                    {$gitState.status.changedFiles.length} changed
                  </span>
                </div>
                {#if $gitState.status.upstream}
                  <p class="mt-1 truncate text-xs text-[var(--text-muted)]">
                    {$gitState.status.upstream}
                    {#if $gitState.status.ahead > 0 || $gitState.status.behind > 0}
                      · ahead {$gitState.status.ahead} / behind
                      {$gitState.status.behind}
                    {/if}
                  </p>
                {/if}
              </div>

              {#if $gitState.status.changedFiles.length > 0}
                <div class="mb-4 space-y-1">
                  <p class="text-xs font-medium text-[var(--text-muted)]">
                    Changed Files
                  </p>
                  {#each $gitState.status.changedFiles as file}
                    <button
                      type="button"
                      class={`w-full rounded-md px-2 py-1.5 text-left hover:bg-[var(--surface-muted)] ${$gitState.selectedFile?.path === file.path ? "bg-[var(--surface-muted)]" : ""}`}
                      title={file.path}
                      onclick={() => selectGitFile(file)}
                      ondblclick={() => openGitFile(file)}
                    >
                      <div class="flex items-center gap-2">
                        <span
                          class="min-w-0 flex-1 truncate text-sm text-[var(--text-muted)]"
                        >
                          {file.path}
                        </span>
                        <span
                          class="shrink-0 text-xs text-[var(--text-subtle)]"
                        >
                          {file.kind}
                        </span>
                      </div>
                      <p
                        class="mt-0.5 truncate text-xs text-[var(--text-subtle)]"
                      >
                        {gitFileBadge(file)} · {file.indexStatus}{file.worktreeStatus}
                      </p>
                    </button>
                  {/each}
                </div>

                {#if $gitState.selectedFile}
                  <div
                    class="space-y-2 border-t border-[var(--border-muted)] pt-3"
                  >
                    <p class="text-xs font-medium text-[var(--text-muted)]">
                      Diff Preview
                    </p>
                    <div
                      class="rounded-md border border-dashed border-[var(--border)] bg-[var(--background)] p-2"
                    >
                      <p class="truncate text-xs text-[var(--text-muted)]">
                        {$gitState.selectedFile.path}
                      </p>
                      <p class="mt-1 text-xs text-[var(--text-subtle)]">
                        Inline diff view will be added after the v0 status
                        panel.
                      </p>
                    </div>
                    <button
                      type="button"
                      class="h-8 w-full rounded-md px-2 text-left text-sm text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)] disabled:cursor-not-allowed disabled:opacity-60"
                      disabled={!canOpenGitFile($gitState.selectedFile)}
                      onclick={() =>
                        $gitState.selectedFile &&
                        openGitFile($gitState.selectedFile)}
                    >
                      Open file
                    </button>
                  </div>
                {/if}
              {:else}
                <p class="text-xs text-[var(--text-subtle)]">
                  Working tree clean.
                </p>
              {/if}
            {/if}
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
                <div class="flex items-center gap-1">
                  <button
                    type="button"
                    class="rounded-md px-2 py-1 text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                    onclick={() => editorController?.find()}
                  >
                    Find
                  </button>
                  <button
                    type="button"
                    class="rounded-md px-2 py-1 text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                    onclick={() => editorController?.replace()}
                  >
                    Replace
                  </button>
                  <button
                    type="button"
                    class="rounded-md px-2 py-1 text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                    onclick={() => editorController?.goToLine()}
                  >
                    Go
                  </button>
                  <button
                    type="button"
                    class={`rounded-md px-2 py-1 ${$editorPreferences.minimap ? "bg-[var(--surface-muted)] text-[var(--text)]" : "text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"}`}
                    onclick={() => {
                      toggleEditorMinimap();
                      editorController?.toggleMinimap();
                    }}
                  >
                    Minimap
                  </button>
                  <button
                    type="button"
                    class={`rounded-md px-2 py-1 ${$editorPreferences.wordWrap ? "bg-[var(--surface-muted)] text-[var(--text)]" : "text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"}`}
                    onclick={() => {
                      toggleEditorWordWrap();
                      editorController?.toggleWordWrap();
                    }}
                  >
                    Wrap
                  </button>
                  <button
                    type="button"
                    class="rounded-md px-2 py-1 text-[var(--text-muted)] hover:bg-[var(--surface-muted)] hover:text-[var(--text)]"
                    onclick={() => editorController?.formatDocument()}
                  >
                    Format
                  </button>
                  <button
                    type="button"
                    class="rounded-md border border-[var(--border)] px-3 py-1 text-[var(--text-muted)] hover:border-[var(--accent)] hover:text-[var(--text)]"
                    onclick={saveActiveDocument}
                  >
                    Save
                  </button>
                </div>
              </div>
              <MonacoEditor
                path={$activeDocument.path}
                content={$activeDocument.content}
                theme={$editorPreferences.theme}
                minimap={$editorPreferences.minimap}
                wordWrap={$editorPreferences.wordWrap}
                onChange={updateActiveDocument}
                onReady={(controller) => {
                  editorController = controller;
                }}
              />
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

          {#if $panelState.bottomView === "terminal"}
            <div class="h-[calc(100%-2.25rem)] min-h-0">
              <TerminalPanel cwd={$workspace.rootPath} />
            </div>
          {:else}
            <div
              class="h-[calc(100%-2.25rem)] p-3 font-mono text-xs text-[var(--text-muted)]"
            >
              {#if $panelState.bottomView === "problems"}
                <p>No problems detected.</p>
              {:else}
                <div class="space-y-2">
                  <p>Workspace events and command output will appear here.</p>
                  <div class="flex flex-wrap gap-2">
                    <button
                      type="button"
                      class="rounded border border-[var(--border)] px-2 py-1 hover:border-[var(--accent)]"
                      onclick={saveAllDocuments}
                    >
                      Save all
                    </button>
                    <button
                      type="button"
                      class="rounded border border-[var(--border)] px-2 py-1 hover:border-[var(--accent)]"
                      onclick={() =>
                        $activeTab &&
                        closeOtherTabsWithConfirmation($activeTab.id)}
                    >
                      Close other tabs
                    </button>
                    <button
                      type="button"
                      class="rounded border border-[var(--border)] px-2 py-1 hover:border-[var(--accent)]"
                      onclick={closeAllTabsWithConfirmation}
                    >
                      Close all tabs
                    </button>
                    <button
                      type="button"
                      class="rounded border border-[var(--border)] px-2 py-1 hover:border-[var(--accent)]"
                      onclick={reopenRecentlyClosedTab}
                    >
                      Reopen closed tab
                    </button>
                  </div>
                </div>
              {/if}
            </div>
          {/if}
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
        onclick={() => setActiveView("git")}
      >
        {gitIndicatorLabel()}
      </button>
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

<CommandLauncher
  open={commandPaletteOpen}
  title="Command Palette"
  placeholder="Search commands"
  items={commandPaletteItems}
  onClose={() => {
    commandPaletteOpen = false;
  }}
  onSelect={selectCommand}
/>

<CommandLauncher
  open={quickOpenOpen}
  title="Quick Open"
  placeholder="Search files"
  items={quickOpenItems}
  onClose={() => {
    quickOpenOpen = false;
  }}
  onSelect={selectQuickOpenFile}
/>
