import { browser } from "$app/environment";
import { derived, get, writable } from "svelte/store";
import { clearGitStatus, refreshGitStatus } from "$lib/stores/git";
import {
  createWorkspaceFile,
  createWorkspaceFolder,
  deleteWorkspaceEntry,
  listWorkspaceTree,
  onWorkspaceChanged,
  pickWorkspaceFolder,
  readWorkspaceFile,
  renameWorkspaceEntry,
  setWindowWorkspaceTitle,
  startWorkspaceWatch,
  writeWorkspaceFile,
  type FileTreeEntry,
  type WorkspaceFsEvent,
} from "$lib/services/workspace";

export type ShellView = "explorer" | "search" | "git" | "project";
export type BottomPanelView = "terminal" | "problems" | "output";

export type WorkspaceState = {
  rootPath: string | null;
  name: string | null;
};

export type EditorTab = {
  id: string;
  title: string;
  path: string;
  dirty: boolean;
  preview: boolean;
};

export type OpenDocument = {
  path: string;
  title: string;
  content: string;
  savedContent: string;
  size: number;
  binary: boolean;
  tooLarge: boolean;
  diskChanged: boolean;
};

export type ClosedTab = {
  tab: EditorTab;
  document: OpenDocument;
};

export type EditorPreferenceState = {
  theme: "larik-dark" | "larik-light";
  minimap: boolean;
  wordWrap: boolean;
};

export type FileTreeState = {
  entries: FileTreeEntry[];
  loading: boolean;
  error: string | null;
  version: number;
};

export type DiskChangeState = {
  path: string;
  kind: WorkspaceFsEvent["kind"];
} | null;

export type PanelState = {
  activeView: ShellView;
  sidebarVisible: boolean;
  bottomVisible: boolean;
  bottomView: BottomPanelView;
  sidebarWidth: number;
  bottomHeight: number;
};

const defaultWorkspace: WorkspaceState = {
  rootPath: null,
  name: null,
};

const defaultTabs: EditorTab[] = [
  {
    id: "welcome",
    title: "Welcome",
    path: "larik://welcome",
    dirty: false,
    preview: false,
  },
];

const defaultPanelState: PanelState = {
  activeView: "explorer",
  sidebarVisible: true,
  bottomVisible: true,
  bottomView: "terminal",
  sidebarWidth: 260,
  bottomHeight: 240,
};

const defaultEditorPreferences: EditorPreferenceState = {
  theme: "larik-dark",
  minimap: false,
  wordWrap: false,
};

let fileTreeLoadId = 0;

function createPersistedStore<T>(key: string, initialValue: T) {
  const storedValue = browser ? localStorage.getItem(key) : null;
  const parsedValue = storedValue
    ? parseStoredValue(storedValue, initialValue)
    : initialValue;
  const store = writable<T>(parsedValue);

  if (browser) {
    store.subscribe((value) => {
      localStorage.setItem(key, JSON.stringify(value));
    });
  }

  return store;
}

function parseStoredValue<T>(value: string, fallback: T) {
  try {
    return JSON.parse(value) as T;
  } catch {
    return fallback;
  }
}

function fileNameFromPath(path: string) {
  return path.split(/[\\/]/).filter(Boolean).at(-1) ?? path;
}

function joinPath(parentPath: string, name: string) {
  return `${parentPath.replace(/[\\/]+$/, "")}/${name}`;
}

function markTabDirty(path: string, dirty: boolean) {
  tabs.update((currentTabs) =>
    currentTabs.map((tab) => (tab.path === path ? { ...tab, dirty } : tab)),
  );
}

function addOrFocusTab(path: string) {
  const existingTab = get(tabs).find((tab) => tab.path === path);

  if (existingTab) {
    activeFile.set(existingTab.id);
    return;
  }

  const tab: EditorTab = {
    id: path,
    title: fileNameFromPath(path),
    path,
    dirty: false,
    preview: false,
  };

  tabs.update((currentTabs) => [...currentTabs, tab]);
  activeFile.set(tab.id);
}

export const workspace = createPersistedStore<WorkspaceState>(
  "larik.workspace.active",
  defaultWorkspace,
);
export const recentWorkspaces = createPersistedStore<string[]>(
  "larik.workspace.recent",
  [],
);
export const tabs = writable<EditorTab[]>(defaultTabs);
export const activeFile = writable<string | null>(defaultTabs[0]?.id ?? null);
export const fileTree = writable<FileTreeState>({
  entries: [],
  loading: false,
  error: null,
  version: 0,
});
export const expandedFolders = createPersistedStore<string[]>(
  "larik.workspace.expandedFolders",
  [],
);
export const documents = writable<Record<string, OpenDocument>>({});
export const recentlyClosedTabs = writable<ClosedTab[]>([]);
export const diskChange = writable<DiskChangeState>(null);
export const panelState = createPersistedStore<PanelState>(
  "larik.shell.panels",
  defaultPanelState,
);
export const editorPreferences = createPersistedStore<EditorPreferenceState>(
  "larik.editor.preferences",
  defaultEditorPreferences,
);

export const activeTab = derived([tabs, activeFile], ([$tabs, $activeFile]) => {
  return $tabs.find((tab) => tab.id === $activeFile) ?? null;
});

export const activeDocument = derived(
  [documents, activeTab],
  ([$documents, $activeTab]) => {
    if (!$activeTab || $activeTab.path === "larik://welcome") {
      return null;
    }

    return $documents[$activeTab.path] ?? null;
  },
);

export function setActiveView(view: ShellView) {
  panelState.update((state) => ({
    ...state,
    activeView: view,
    sidebarVisible: true,
  }));
}

export function setBottomView(view: BottomPanelView) {
  panelState.update((state) => ({
    ...state,
    bottomView: view,
    bottomVisible: true,
  }));
}

export function toggleSidebar() {
  panelState.update((state) => ({
    ...state,
    sidebarVisible: !state.sidebarVisible,
  }));
}

export function toggleBottomPanel() {
  panelState.update((state) => ({
    ...state,
    bottomVisible: !state.bottomVisible,
  }));
}

export function setSidebarWidth(width: number) {
  panelState.update((state) => ({
    ...state,
    sidebarWidth: Math.min(Math.max(width, 200), 420),
  }));
}

export function setBottomHeight(height: number) {
  panelState.update((state) => ({
    ...state,
    bottomHeight: Math.min(Math.max(height, 180), 360),
  }));
}

export function resetShellState() {
  workspace.set(defaultWorkspace);
  tabs.set(defaultTabs);
  activeFile.set(defaultTabs[0]?.id ?? null);
  fileTree.set({ entries: [], loading: false, error: null, version: 0 });
  expandedFolders.set([]);
  documents.set({});
  recentlyClosedTabs.set([]);
  diskChange.set(null);
  clearGitStatus();
  panelState.set(defaultPanelState);
  editorPreferences.set(defaultEditorPreferences);
}

export function getPanelStateSnapshot() {
  return get(panelState);
}

export async function initializeWorkspace() {
  const activeWorkspace = get(workspace);
  await setWindowWorkspaceTitle(activeWorkspace.name);

  if (!activeWorkspace.rootPath) {
    clearGitStatus();
    return;
  }

  const rootPath = activeWorkspace.rootPath;
  expandedFolders.update((folders) =>
    folders.includes(rootPath) ? folders : [rootPath, ...folders],
  );
  await reloadFileTree(rootPath);
  await refreshGitStatus(rootPath);
  await startWorkspaceWatch(rootPath);
}

export async function openWorkspace(path?: string) {
  const selectedPath = path ?? (await pickWorkspaceFolder());

  if (!selectedPath) {
    return;
  }

  const nextWorkspace = {
    rootPath: selectedPath,
    name: fileNameFromPath(selectedPath),
  };

  workspace.set(nextWorkspace);
  recentWorkspaces.update((items) =>
    [selectedPath, ...items.filter((item) => item !== selectedPath)].slice(
      0,
      8,
    ),
  );
  tabs.set(defaultTabs);
  activeFile.set(defaultTabs[0]?.id ?? null);
  documents.set({});
  recentlyClosedTabs.set([]);
  diskChange.set(null);
  clearGitStatus();
  fileTree.set({ entries: [], loading: true, error: null, version: 0 });
  expandedFolders.set([selectedPath]);
  await setWindowWorkspaceTitle(nextWorkspace.name);
  await reloadFileTree(selectedPath);
  await refreshGitStatus(selectedPath);
  await startWorkspaceWatch(selectedPath);
}

export async function reloadFileTree(rootPathOverride?: string) {
  const rootPath = rootPathOverride ?? get(workspace).rootPath;

  if (!rootPath) {
    fileTree.update((state) => ({
      entries: [],
      loading: false,
      error: null,
      version: state.version + 1,
    }));
    return;
  }

  const loadId = fileTreeLoadId + 1;
  fileTreeLoadId = loadId;
  fileTree.update((state) => ({
    entries: state.entries,
    loading: true,
    error: null,
    version: state.version + 1,
  }));

  try {
    const entries = await listWorkspaceTree(rootPath);
    if (loadId !== fileTreeLoadId || get(workspace).rootPath !== rootPath) {
      return;
    }
    fileTree.update((state) => ({
      entries,
      loading: false,
      error: null,
      version: state.version + 1,
    }));
  } catch (error) {
    if (loadId !== fileTreeLoadId || get(workspace).rootPath !== rootPath) {
      return;
    }
    fileTree.update((state) => ({
      entries: [],
      loading: false,
      error: error instanceof Error ? error.message : String(error),
      version: state.version + 1,
    }));
  }
}

export function toggleFolder(path: string) {
  expandedFolders.update((folders) =>
    folders.includes(path)
      ? folders.filter((folder) => folder !== path)
      : [...folders, path],
  );
  fileTree.update((state) => ({ ...state, version: state.version + 1 }));
}

export async function openFile(path: string) {
  const rootPath = get(workspace).rootPath;

  if (!rootPath) {
    return;
  }

  const response = await readWorkspaceFile(rootPath, path);
  const title = fileNameFromPath(path);

  documents.update((currentDocuments) => ({
    ...currentDocuments,
    [path]: {
      path,
      title,
      content: response.content ?? "",
      savedContent: response.content ?? "",
      size: response.size,
      binary: response.binary,
      tooLarge: response.tooLarge,
      diskChanged: false,
    },
  }));
  addOrFocusTab(path);
}

export function updateActiveDocument(content: string) {
  const tab = get(activeTab);

  if (!tab || tab.path === "larik://welcome") {
    return;
  }

  documents.update((currentDocuments) => {
    const document = currentDocuments[tab.path];

    if (!document) {
      return currentDocuments;
    }

    const nextDocument = {
      ...document,
      content,
    };
    markTabDirty(tab.path, nextDocument.content !== nextDocument.savedContent);

    return {
      ...currentDocuments,
      [tab.path]: nextDocument,
    };
  });
}

export async function saveActiveDocument() {
  const rootPath = get(workspace).rootPath;
  const document = get(activeDocument);

  if (!rootPath || !document || document.binary || document.tooLarge) {
    return;
  }

  await writeWorkspaceFile(rootPath, document.path, document.content);
  await refreshGitStatus(rootPath);
  documents.update((currentDocuments) => ({
    ...currentDocuments,
    [document.path]: {
      ...document,
      savedContent: document.content,
      diskChanged: false,
    },
  }));
  markTabDirty(document.path, false);
}

export async function saveAllDocuments() {
  const rootPath = get(workspace).rootPath;

  if (!rootPath) {
    return;
  }

  const openDocuments = Object.values(get(documents)).filter(
    (document) =>
      !document.binary &&
      !document.tooLarge &&
      document.content !== document.savedContent,
  );

  for (const document of openDocuments) {
    await writeWorkspaceFile(rootPath, document.path, document.content);
    await refreshGitStatus(rootPath);
    documents.update((currentDocuments) => ({
      ...currentDocuments,
      [document.path]: {
        ...currentDocuments[document.path],
        savedContent: document.content,
        diskChanged: false,
      },
    }));
    markTabDirty(document.path, false);
  }
}

export function closeTab(tabId: string) {
  const targetTab = get(tabs).find((tab) => tab.id === tabId);
  const targetDocument = get(documents)[targetTab?.path ?? ""];

  if (!targetTab || targetTab.dirty) {
    return false;
  }

  if (targetDocument) {
    recentlyClosedTabs.update((closedTabs) =>
      [{ tab: targetTab, document: targetDocument }, ...closedTabs].slice(
        0,
        10,
      ),
    );
  }

  tabs.update((currentTabs) => {
    const nextTabs = currentTabs.filter((tab) => tab.id !== tabId);
    return nextTabs.length > 0 ? nextTabs : defaultTabs;
  });
  documents.update((currentDocuments) => {
    const { [targetTab.path]: _closed, ...remainingDocuments } =
      currentDocuments;
    return remainingDocuments;
  });

  const nextTabs = get(tabs);
  if (!nextTabs.some((tab) => tab.id === get(activeFile))) {
    activeFile.set(nextTabs[0]?.id ?? null);
  }

  return true;
}

export function closeOtherTabs(tabId: string) {
  const activeTabs = get(tabs);
  const targetTab = activeTabs.find((tab) => tab.id === tabId);

  if (!targetTab) {
    return false;
  }

  const closingTabs = activeTabs.filter(
    (tab) => tab.id !== tabId && tab.id !== "welcome",
  );

  if (closingTabs.some((tab) => tab.dirty)) {
    return false;
  }

  const currentDocuments = get(documents);
  recentlyClosedTabs.update((closedTabs) =>
    [
      ...closingTabs
        .map((tab) =>
          currentDocuments[tab.path]
            ? { tab, document: currentDocuments[tab.path] }
            : null,
        )
        .filter((item): item is ClosedTab => Boolean(item)),
      ...closedTabs,
    ].slice(0, 10),
  );
  tabs.set([targetTab]);
  activeFile.set(targetTab.id);
  documents.set(
    targetTab.path === "larik://welcome" || !currentDocuments[targetTab.path]
      ? {}
      : { [targetTab.path]: currentDocuments[targetTab.path] },
  );
  return true;
}

export function closeAllTabs() {
  const activeTabs = get(tabs).filter((tab) => tab.id !== "welcome");

  if (activeTabs.some((tab) => tab.dirty)) {
    return false;
  }

  const currentDocuments = get(documents);
  recentlyClosedTabs.update((closedTabs) =>
    [
      ...activeTabs
        .map((tab) =>
          currentDocuments[tab.path]
            ? { tab, document: currentDocuments[tab.path] }
            : null,
        )
        .filter((item): item is ClosedTab => Boolean(item)),
      ...closedTabs,
    ].slice(0, 10),
  );
  tabs.set(defaultTabs);
  activeFile.set(defaultTabs[0]?.id ?? null);
  documents.set({});
  return true;
}

export function reopenRecentlyClosedTab() {
  const [closedTab, ...remainingTabs] = get(recentlyClosedTabs);

  if (!closedTab) {
    return;
  }

  documents.update((currentDocuments) => ({
    ...currentDocuments,
    [closedTab.document.path]: closedTab.document,
  }));
  tabs.update((currentTabs) =>
    currentTabs.some((tab) => tab.id === closedTab.tab.id)
      ? currentTabs
      : [...currentTabs.filter((tab) => tab.id !== "welcome"), closedTab.tab],
  );
  activeFile.set(closedTab.tab.id);
  recentlyClosedTabs.set(remainingTabs);
}

export function setEditorTheme(theme: EditorPreferenceState["theme"]) {
  editorPreferences.update((preferences) => ({ ...preferences, theme }));
}

export function toggleEditorMinimap() {
  editorPreferences.update((preferences) => ({
    ...preferences,
    minimap: !preferences.minimap,
  }));
}

export function toggleEditorWordWrap() {
  editorPreferences.update((preferences) => ({
    ...preferences,
    wordWrap: !preferences.wordWrap,
  }));
}

export async function createEntry(
  parentPath: string,
  name: string,
  type: "file" | "folder",
) {
  const rootPath = get(workspace).rootPath;

  if (!rootPath) {
    return;
  }

  const path = joinPath(parentPath, name);
  if (type === "folder") {
    await createWorkspaceFolder(rootPath, path);
    expandedFolders.update((folders) => [...new Set([...folders, path])]);
  } else {
    await createWorkspaceFile(rootPath, path);
    await openFile(path);
  }

  await reloadFileTree(rootPath);
  await refreshGitStatus(rootPath);
}

export async function renameEntry(path: string, newName: string) {
  const rootPath = get(workspace).rootPath;

  if (!rootPath) {
    return;
  }

  await renameWorkspaceEntry(rootPath, path, newName);
  await reloadFileTree(rootPath);
  await refreshGitStatus(rootPath);
}

export async function deleteEntry(path: string) {
  const rootPath = get(workspace).rootPath;

  if (!rootPath) {
    return;
  }

  await deleteWorkspaceEntry(rootPath, path);
  tabs.update((currentTabs) =>
    currentTabs.filter(
      (tab) => tab.path !== path && !tab.path.startsWith(`${path}/`),
    ),
  );
  documents.update((currentDocuments) => {
    return Object.fromEntries(
      Object.entries(currentDocuments).filter(
        ([documentPath]) =>
          documentPath !== path && !documentPath.startsWith(`${path}/`),
      ),
    );
  });
  if (get(tabs).length === 0) {
    tabs.set(defaultTabs);
  }
  activeFile.set(get(tabs)[0]?.id ?? null);
  await reloadFileTree(rootPath);
  await refreshGitStatus(rootPath);
}

export async function reloadDiskChangedDocument() {
  const change = get(diskChange);

  if (!change) {
    return;
  }

  await openFile(change.path);
  diskChange.set(null);
}

export function keepLocalDocument() {
  const change = get(diskChange);

  if (!change) {
    return;
  }

  documents.update((currentDocuments) => {
    const document = currentDocuments[change.path];

    if (!document) {
      return currentDocuments;
    }

    return {
      ...currentDocuments,
      [change.path]: {
        ...document,
        diskChanged: false,
      },
    };
  });
  diskChange.set(null);
}

export async function registerWorkspaceWatcher() {
  if (!browser) {
    return;
  }

  await onWorkspaceChanged(async (event) => {
    const rootPath = get(workspace).rootPath;
    if (rootPath) {
      await reloadFileTree(rootPath);
      await refreshGitStatus(rootPath);
    }

    const openDocuments = get(documents);
    const changedPath = event.paths.find((path) => openDocuments[path]);

    if (!changedPath) {
      return;
    }

    const document = openDocuments[changedPath];

    if (
      event.kind === "removed" ||
      document.content !== document.savedContent
    ) {
      documents.update((currentDocuments) => ({
        ...currentDocuments,
        [changedPath]: {
          ...currentDocuments[changedPath],
          diskChanged: true,
        },
      }));
      diskChange.set({ path: changedPath, kind: event.kind });
      return;
    }

    await openFile(changedPath);
  });
}
