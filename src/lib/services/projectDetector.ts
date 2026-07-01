import { invoke } from "@tauri-apps/api/core";

export type DetectedProject = {
  kind: string;
  name: string;
  confidence: number;
  detectedFiles: string[];
  details: Record<string, unknown>;
};

export type ProjectAction = {
  id: string;
  label: string;
  command: string;
  cwd: string;
  category: string;
  destructive: boolean;
};

export type EnvStatus = {
  hasEnv: boolean;
  hasEnvExample: boolean;
  missingKeys: string[];
  emptyKeys: string[];
};

export type ProjectDetectionResponse = {
  rootPath: string;
  detected: DetectedProject[];
  actions: ProjectAction[];
  env: EnvStatus;
  warnings: string[];
};

export function detectProject(rootPath: string) {
  return invoke<ProjectDetectionResponse>("detect_project", { rootPath });
}
