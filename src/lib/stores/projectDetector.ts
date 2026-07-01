import { writable } from "svelte/store";
import {
  detectProject,
  type ProjectDetectionResponse,
} from "$lib/services/projectDetector";

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
}
