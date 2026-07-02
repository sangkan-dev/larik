import { get, writable } from "svelte/store";
import {
  changeLspDocument,
  closeLspDocument,
  lspLanguageFromPath,
  onLspDiagnostics,
  openLspDocument,
  saveLspDocument,
  startLsp,
  type LspDiagnostic,
  type LspServerInfo,
} from "$lib/services/lsp";

export type LspState = {
  servers: Record<string, LspServerInfo>;
  diagnostics: Record<string, LspDiagnostic[]>;
  documents: Record<string, { languageId: string; version: number }>;
  loading: boolean;
  error: string | null;
};

export const lspState = writable<LspState>({
  servers: {},
  diagnostics: {},
  documents: {},
  loading: false,
  error: null,
});

let diagnosticsRegistered = false;
const startingServers = new Set<string>();

function serverKey(rootPath: string, languageId: string) {
  return `${rootPath}::${languageId}`;
}

function documentKey(rootPath: string, path: string) {
  return `${rootPath}::${path}`;
}

export function registerLspEvents() {
  if (diagnosticsRegistered) {
    return;
  }
  diagnosticsRegistered = true;
  onLspDiagnostics((event) => {
    if (!event.path) {
      return;
    }
    lspState.update((state) => ({
      ...state,
      diagnostics: {
        ...state.diagnostics,
        [event.path ?? event.uri]: event.diagnostics,
      },
    }));
  });
}

export async function ensureLspDocument(
  rootPath: string,
  path: string,
  content: string,
) {
  const languageId = lspLanguageFromPath(path);
  if (!languageId) {
    return;
  }

  const serverReady = await ensureLspServer(rootPath, languageId);
  if (!serverReady) {
    return;
  }
  const key = documentKey(rootPath, path);
  const existing = get(lspState).documents[key];
  if (existing) {
    return;
  }

  await openLspDocument(rootPath, languageId, path, content, 1);
  lspState.update((state) => ({
    ...state,
    documents: {
      ...state.documents,
      [key]: { languageId, version: 1 },
    },
  }));
}

export async function syncLspDocumentChange(
  rootPath: string,
  path: string,
  content: string,
) {
  const languageId = lspLanguageFromPath(path);
  if (!languageId) {
    return;
  }

  await ensureLspDocument(rootPath, path, content);
  const key = documentKey(rootPath, path);
  const current = get(lspState).documents[key];
  if (!current) {
    return;
  }
  const version = (current?.version ?? 1) + 1;
  await changeLspDocument(rootPath, languageId, path, content, version);
  lspState.update((state) => ({
    ...state,
    documents: {
      ...state.documents,
      [key]: { languageId, version },
    },
  }));
}

export async function syncLspDocumentSave(
  rootPath: string,
  path: string,
  content: string,
) {
  const languageId = lspLanguageFromPath(path);
  if (!languageId) {
    return;
  }

  const current = get(lspState).documents[documentKey(rootPath, path)];
  if (!current) {
    return;
  }
  await saveLspDocument(
    rootPath,
    languageId,
    path,
    content,
    current?.version ?? 1,
  );
}

export async function closeSyncedLspDocument(rootPath: string, path: string) {
  const key = documentKey(rootPath, path);
  const current = get(lspState).documents[key];
  if (!current) {
    return;
  }

  await closeLspDocument(rootPath, current.languageId, path);
  lspState.update((state) => {
    const documents = { ...state.documents };
    delete documents[key];
    return { ...state, documents };
  });
}

export function diagnosticsForPath(path: string | null) {
  return path ? (get(lspState).diagnostics[path] ?? []) : [];
}

export function lspStatusLabel(activePath: string | null) {
  if (!activePath) {
    return "LSP idle";
  }

  const languageId = lspLanguageFromPath(activePath);
  if (!languageId) {
    return "LSP unavailable";
  }

  const state = get(lspState);
  if (state.loading) {
    return "LSP starting";
  }
  if (state.error) {
    return "LSP error";
  }

  const serverRunning = Object.keys(state.servers).some((key) =>
    key.endsWith(`::${languageId}`),
  );
  return serverRunning ? `LSP ${languageId}` : "LSP idle";
}

async function ensureLspServer(rootPath: string, languageId: string) {
  const key = serverKey(rootPath, languageId);
  const state = get(lspState);
  if (state.servers[key] || startingServers.has(key)) {
    return true;
  }

  startingServers.add(key);
  lspState.update((current) => ({ ...current, loading: true, error: null }));
  try {
    const server = await startLsp(rootPath, languageId);
    lspState.update((current) => ({
      ...current,
      loading: false,
      error: null,
      servers: {
        ...current.servers,
        [key]: server,
      },
    }));
    return true;
  } catch (error) {
    lspState.update((current) => ({
      ...current,
      loading: false,
      error: error instanceof Error ? error.message : String(error),
    }));
    return false;
  } finally {
    startingServers.delete(key);
  }
}
