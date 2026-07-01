import { writable } from "svelte/store";
import {
  detectProject,
  type ProjectAction,
  type ProjectDetectionResponse,
} from "$lib/services/projectDetector";
import type { TerminalExitEvent } from "$lib/services/terminal";

export type ProjectDetectionState = {
  result: ProjectDetectionResponse | null;
  loading: boolean;
  error: string | null;
};

export const projectDetection = writable<ProjectDetectionState>({
  result: null,
  loading: false,
  error: null,
});

export type ProjectActionRunStatus = "running" | "done" | "failed";

export type ProjectActionRun = {
  actionId: string;
  sessionId: string | null;
  label: string;
  command: string;
  cwd: string;
  status: ProjectActionRunStatus;
  exitCode: number | null;
  error: string | null;
  startedAt: number;
  finishedAt: number | null;
};

export const projectActionRuns = writable<Record<string, ProjectActionRun>>({});

let activeScan = 0;

export async function scanProject(rootPath: string) {
  const scanId = activeScan + 1;
  activeScan = scanId;
  projectDetection.set({ result: null, loading: true, error: null });

  try {
    const result = await detectProject(rootPath);
    if (scanId === activeScan) {
      projectDetection.set({ result, loading: false, error: null });
    }
  } catch (error) {
    if (scanId === activeScan) {
      projectDetection.set({
        result: null,
        loading: false,
        error: error instanceof Error ? error.message : String(error),
      });
    }
  }
}

export function clearProjectDetection() {
  activeScan += 1;
  projectDetection.set({ result: null, loading: false, error: null });
  projectActionRuns.set({});
}

export function markProjectActionRunning(
  action: ProjectAction,
  sessionId: string,
) {
  projectActionRuns.update((runs) => ({
    ...runs,
    [action.id]: {
      actionId: action.id,
      sessionId,
      label: action.label,
      command: action.command,
      cwd: action.cwd,
      status: "running",
      exitCode: null,
      error: null,
      startedAt: Date.now(),
      finishedAt: null,
    },
  }));
}

export function markProjectActionFailed(action: ProjectAction, error: string) {
  projectActionRuns.update((runs) => ({
    ...runs,
    [action.id]: {
      actionId: action.id,
      sessionId: null,
      label: action.label,
      command: action.command,
      cwd: action.cwd,
      status: "failed",
      exitCode: null,
      error,
      startedAt: Date.now(),
      finishedAt: Date.now(),
    },
  }));
}

export function markProjectActionTerminalExit(event: TerminalExitEvent) {
  projectActionRuns.update((runs) => {
    const matchingRun = Object.values(runs).find(
      (run) => run.sessionId === event.sessionId && run.status === "running",
    );

    if (!matchingRun) {
      return runs;
    }

    return {
      ...runs,
      [matchingRun.actionId]: {
        ...matchingRun,
        status: event.success ? "done" : "failed",
        exitCode: event.exitCode,
        error: event.success ? null : `Exited with code ${event.exitCode}`,
        finishedAt: Date.now(),
      },
    };
  });
}
