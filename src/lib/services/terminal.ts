import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type SpawnTerminalRequest = {
  cwd?: string | null;
  rows?: number;
  cols?: number;
  command?: string | null;
  label?: string | null;
};

export type SpawnTerminalResponse = {
  sessionId: string;
  label: string;
};

export type TerminalOutputEvent = {
  sessionId: string;
  data: string;
};

export type TerminalExitEvent = {
  sessionId: string;
};

export function spawnTerminal(request: SpawnTerminalRequest) {
  return invoke<SpawnTerminalResponse>("terminal_spawn", { request });
}

export function writeTerminal(sessionId: string, data: string) {
  return invoke<void>("terminal_write", {
    request: {
      sessionId,
      data,
    },
  });
}

export function resizeTerminal(sessionId: string, rows: number, cols: number) {
  return invoke<void>("terminal_resize", {
    request: {
      sessionId,
      rows,
      cols,
    },
  });
}

export function killTerminal(sessionId: string) {
  return invoke<void>("terminal_kill", { sessionId });
}

export function onTerminalOutput(
  callback: (event: TerminalOutputEvent) => void,
) {
  return listen<TerminalOutputEvent>("terminal://output", (event) => {
    callback(event.payload);
  });
}

export function onTerminalExit(callback: (event: TerminalExitEvent) => void) {
  return listen<TerminalExitEvent>("terminal://exit", (event) => {
    callback(event.payload);
  });
}
