import { browser } from "$app/environment";
import { derived, get, writable } from "svelte/store";

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

function createPersistedStore<T>(key: string, initialValue: T) {
  const storedValue = browser ? localStorage.getItem(key) : null;
  const parsedValue = storedValue
    ? (JSON.parse(storedValue) as T)
    : initialValue;
  const store = writable<T>(parsedValue);

  if (browser) {
    store.subscribe((value) => {
      localStorage.setItem(key, JSON.stringify(value));
    });
  }

  return store;
}

export const workspace = writable<WorkspaceState>(defaultWorkspace);
export const tabs = writable<EditorTab[]>(defaultTabs);
export const activeFile = writable<string | null>(defaultTabs[0]?.id ?? null);
export const panelState = createPersistedStore<PanelState>(
  "larik.shell.panels",
  defaultPanelState,
);

export const activeTab = derived([tabs, activeFile], ([$tabs, $activeFile]) => {
  return $tabs.find((tab) => tab.id === $activeFile) ?? null;
});

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
  panelState.set(defaultPanelState);
}

export function getPanelStateSnapshot() {
  return get(panelState);
}
