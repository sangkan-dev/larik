import { writable } from "svelte/store";
import {
  commitGitChanges,
  generateGitCommitMessage,
  getGitDiff,
  getGitStatus,
  stageGitFile,
  unstageGitFile,
  type GitChangedFile,
  type GitDiffResponse,
  type GitStatusResponse,
} from "$lib/services/git";

export type GitState = {
  status: GitStatusResponse | null;
  loading: boolean;
  error: string | null;
  selectedFile: GitChangedFile | null;
  diff: GitDiffResponse | null;
  diffLoading: boolean;
  operation: string | null;
  commitMessage: string;
};

export const gitState = writable<GitState>({
  status: null,
  loading: false,
  error: null,
  selectedFile: null,
  diff: null,
  diffLoading: false,
  operation: null,
  commitMessage: "",
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
      selectedFile: nextSelectedFile(status, state.selectedFile),
      diff: state.diff,
      diffLoading: state.diffLoading,
      operation: state.operation,
      commitMessage: state.commitMessage,
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

export async function loadGitDiff(
  rootPath: string,
  file: GitChangedFile,
  staged = file.staged && !file.unstaged,
) {
  gitState.update((state) => ({
    ...state,
    selectedFile: file,
    diffLoading: true,
    error: null,
  }));

  try {
    let diff = await getGitDiff(rootPath, file.path, staged);
    if (!diff.content && file.staged && file.unstaged) {
      const fallback = await getGitDiff(rootPath, file.path, !staged);
      if (fallback.content) {
        diff = fallback;
      }
    }
    if (!diff.content && file.staged !== file.unstaged) {
      const fallback = await getGitDiff(rootPath, file.path, !staged);
      if (fallback.content) {
        diff = fallback;
      }
    }
    gitState.update((state) => ({
      ...state,
      diff,
      diffLoading: false,
      error: null,
    }));
  } catch (error) {
    gitState.update((state) => ({
      ...state,
      diff: null,
      diffLoading: false,
      error: error instanceof Error ? error.message : String(error),
    }));
  }
}

export async function stageFile(rootPath: string, file: GitChangedFile) {
  await runGitOperation("stage", async () => stageGitFile(rootPath, file.path));
}

export async function unstageFile(rootPath: string, file: GitChangedFile) {
  await runGitOperation("unstage", async () =>
    unstageGitFile(rootPath, file.path),
  );
}

export async function commitChanges(rootPath: string, message: string) {
  await runGitOperation("commit", async () =>
    commitGitChanges(rootPath, message),
  );
}

export async function generateCommitMessage(rootPath: string) {
  gitState.update((state) => ({ ...state, operation: "message", error: null }));

  try {
    const message = await generateGitCommitMessage(rootPath);
    gitState.update((state) => ({
      ...state,
      commitMessage: message,
      operation: null,
    }));
  } catch (error) {
    gitState.update((state) => ({
      ...state,
      operation: null,
      error: error instanceof Error ? error.message : String(error),
    }));
  }
}

async function runGitOperation(
  operation: string,
  task: () => Promise<GitStatusResponse>,
) {
  gitState.update((state) => ({ ...state, operation, error: null }));

  try {
    const status = await task();
    gitState.update((state) => ({
      ...state,
      status,
      selectedFile: nextSelectedFile(status, state.selectedFile),
      diff: null,
      diffLoading: false,
      operation: null,
      error: null,
      commitMessage: operation === "commit" ? "" : state.commitMessage,
    }));
  } catch (error) {
    gitState.update((state) => ({
      ...state,
      operation: null,
      error: error instanceof Error ? error.message : String(error),
    }));
  }
}

function nextSelectedFile(
  status: GitStatusResponse,
  selectedFile: GitChangedFile | null,
) {
  return (
    (selectedFile &&
      status.changedFiles.find((file) => file.path === selectedFile.path)) ||
    status.changedFiles[0] ||
    null
  );
}

export function setCommitMessage(message: string) {
  gitState.update((state) => ({ ...state, commitMessage: message }));
}

export function clearGitStatus() {
  activeScan += 1;
  gitState.set({
    status: null,
    loading: false,
    error: null,
    selectedFile: null,
    diff: null,
    diffLoading: false,
    operation: null,
    commitMessage: "",
  });
}

export function selectGitFile(file: GitChangedFile) {
  gitState.update((state) => ({ ...state, selectedFile: file, diff: null }));
}
