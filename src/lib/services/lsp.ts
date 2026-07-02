import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type LspServerInfo = {
  languageId: string;
  running: boolean;
  command: string;
};

export type LspRange = {
  start: { line: number; character: number };
  end: { line: number; character: number };
};

export type LspDiagnostic = {
  range: LspRange;
  severity?: number;
  code?: string | number;
  source?: string;
  message: string;
};

export type LspDiagnosticsEvent = {
  languageId: string;
  uri: string;
  path: string | null;
  diagnostics: LspDiagnostic[];
};

export type LspPosition = {
  rootPath: string;
  languageId: string;
  path: string;
  line: number;
  character: number;
};

export function lspLanguageFromPath(path: string) {
  const fileName = path.split(/[\\/]/).at(-1)?.toLowerCase() ?? "";
  const extension = fileName.split(".").at(-1) ?? "";

  if (fileName.endsWith(".svelte")) return "svelte";
  if (extension === "ts" || extension === "tsx") return "typescript";
  if (extension === "js" || extension === "jsx" || extension === "cjs") {
    return "javascript";
  }
  if (extension === "rs") return "rust";
  if (extension === "go") return "go";
  if (extension === "php") return "php";

  return null;
}

export function startLsp(rootPath: string, languageId: string) {
  return invoke<LspServerInfo>("lsp_start", {
    request: { rootPath, languageId },
  });
}

export function stopLsp(rootPath: string, languageId: string) {
  return invoke<void>("lsp_stop", {
    request: { rootPath, languageId },
  });
}

export function openLspDocument(
  rootPath: string,
  languageId: string,
  path: string,
  content: string,
  version: number,
) {
  return invoke<void>("lsp_document_open", {
    request: { rootPath, languageId, path, content, version },
  });
}

export function changeLspDocument(
  rootPath: string,
  languageId: string,
  path: string,
  content: string,
  version: number,
) {
  return invoke<void>("lsp_document_change", {
    request: { rootPath, languageId, path, content, version },
  });
}

export function saveLspDocument(
  rootPath: string,
  languageId: string,
  path: string,
  content: string,
  version: number,
) {
  return invoke<void>("lsp_document_save", {
    request: { rootPath, languageId, path, content, version },
  });
}

export function closeLspDocument(
  rootPath: string,
  languageId: string,
  path: string,
) {
  return invoke<void>("lsp_document_close", {
    request: { rootPath, languageId, path },
  });
}

export function requestLspCompletion(position: LspPosition) {
  return invoke<unknown>("lsp_completion", { request: position });
}

export function requestLspHover(position: LspPosition) {
  return invoke<unknown>("lsp_hover", { request: position });
}

export function requestLspDefinition(position: LspPosition) {
  return invoke<unknown>("lsp_definition", { request: position });
}

export function requestLspFormatting(
  rootPath: string,
  languageId: string,
  path: string,
) {
  return invoke<unknown>("lsp_formatting", {
    request: { rootPath, languageId, path },
  });
}

export function onLspDiagnostics(
  callback: (event: LspDiagnosticsEvent) => void,
) {
  return listen<LspDiagnosticsEvent>("lsp://diagnostics", (event) => {
    callback(event.payload);
  });
}
