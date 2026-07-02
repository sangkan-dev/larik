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

export function getGitStatus(rootPath: string) {
  return invoke<GitStatusResponse>("git_status", { rootPath });
}
