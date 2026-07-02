import { writable } from "svelte/store";
import {
  getGitStatus,
  type GitChangedFile,
  type GitStatusResponse,
} from "$lib/services/git";

export type GitState = {
  status: GitStatusResponse | null;
  loading: boolean;
  error: string | null;
  selectedFile: GitChangedFile | null;
};

export const gitState = writable<GitState>({
  status: null,
  loading: false,
  error: null,
  selectedFile: null,
});

let activeScan = 0;

export async function refreshGitStatus(rootPath: string) {
  const scanId = activeScan + 1;
  activeScan = scanId;
  gitState.update((state) => ({ ...state, loading: true, error: null }));

  try {
    const status = await getGitStatus(rootPath);
    if (scanId !== activeScan) {
      return;
    }

    gitState.update((state) => ({
      status,
      loading: false,
      error: null,
      selectedFile:
        state.selectedFile &&
        status.changedFiles.some(
          (file) => file.path === state.selectedFile?.path,
        )
          ? state.selectedFile
          : (status.changedFiles[0] ?? null),
    }));
  } catch (error) {
    if (scanId === activeScan) {
      gitState.update((state) => ({
        ...state,
        loading: false,
        error: error instanceof Error ? error.message : String(error),
      }));
    }
  }
}

export function clearGitStatus() {
  activeScan += 1;
  gitState.set({
    status: null,
    loading: false,
    error: null,
    selectedFile: null,
  });
}

export function selectGitFile(file: GitChangedFile) {
  gitState.update((state) => ({ ...state, selectedFile: file }));
}
