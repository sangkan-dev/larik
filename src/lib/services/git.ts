import { invoke } from "@tauri-apps/api/core";

export type GitChangedFile = {
  path: string;
  absolutePath: string;
  staged: boolean;
  unstaged: boolean;
  untracked: boolean;
  kind: string;
  indexStatus: string;
  worktreeStatus: string;
};

export type GitStatusResponse = {
  rootPath: string;
  isRepo: boolean;
  branch: string | null;
  upstream: string | null;
  ahead: number;
  behind: number;
  changedFiles: GitChangedFile[];
};

export type GitDiffResponse = {
  path: string;
  staged: boolean;
  content: string;
};

export function getGitStatus(rootPath: string) {
  return invoke<GitStatusResponse>("git_status", { rootPath });
}

export function stageGitFile(rootPath: string, path: string) {
  return invoke<GitStatusResponse>("git_stage_file", {
    request: { rootPath, path },
  });
}

export function unstageGitFile(rootPath: string, path: string) {
  return invoke<GitStatusResponse>("git_unstage_file", {
    request: { rootPath, path },
  });
}

export function commitGitChanges(rootPath: string, message: string) {
  return invoke<GitStatusResponse>("git_commit", {
    request: { rootPath, message },
  });
}

export function getGitDiff(rootPath: string, path: string, staged: boolean) {
  return invoke<GitDiffResponse>("git_diff_file", {
    request: { rootPath, path, staged },
  });
}

export function generateGitCommitMessage(rootPath: string) {
  return invoke<string>("git_generate_commit_message", { rootPath });
}
