import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";

export type FileTreeEntry = {
  name: string;
  path: string;
  isDir: boolean;
  children?: FileTreeEntry[];
};

export type ReadFileResponse = {
  path: string;
  content: string | null;
  size: number;
  tooLarge: boolean;
  binary: boolean;
};

export type WorkspaceFsEvent = {
  kind: "created" | "modified" | "removed" | "changed";
  paths: string[];
};

export async function pickWorkspaceFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Open Folder",
  });

  return typeof selected === "string" ? selected : null;
}

export async function setWindowWorkspaceTitle(workspaceName: string | null) {
  await getCurrentWindow().setTitle(
    workspaceName ? `Larik - ${workspaceName}` : "Larik",
  );
}

export function listWorkspaceTree(rootPath: string) {
  return invoke<FileTreeEntry[]>("list_workspace_tree", { rootPath });
}

export function readWorkspaceFile(rootPath: string, path: string) {
  return invoke<ReadFileResponse>("read_workspace_file", { rootPath, path });
}

export function writeWorkspaceFile(
  rootPath: string,
  path: string,
  content: string,
) {
  return invoke<void>("write_workspace_file", { rootPath, path, content });
}

export function createWorkspaceFile(rootPath: string, path: string) {
  return invoke<void>("create_workspace_file", { rootPath, path });
}

export function createWorkspaceFolder(rootPath: string, path: string) {
  return invoke<void>("create_workspace_folder", { rootPath, path });
}

export function renameWorkspaceEntry(
  rootPath: string,
  path: string,
  newName: string,
) {
  return invoke<void>("rename_workspace_entry", { rootPath, path, newName });
}

export function deleteWorkspaceEntry(rootPath: string, path: string) {
  return invoke<void>("delete_workspace_entry", { rootPath, path });
}

export function startWorkspaceWatch(rootPath: string) {
  return invoke<void>("start_workspace_watch", { rootPath });
}

export function onWorkspaceChanged(
  callback: (event: WorkspaceFsEvent) => void,
) {
  return listen<WorkspaceFsEvent>("workspace://changed", (event) => {
    callback(event.payload);
  });
}
