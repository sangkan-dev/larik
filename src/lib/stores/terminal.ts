import { browser } from "$app/environment";
import { get, writable } from "svelte/store";
import {
  killTerminal,
  onTerminalExit,
  onTerminalOutput,
  resizeTerminal,
  spawnTerminal,
  writeTerminal,
} from "$lib/services/terminal";
import { markProjectActionTerminalExit } from "$lib/stores/projectDetector";

export type TerminalSession = {
  id: string;
  label: string;
  cwd: string | null;
  active: boolean;
  exited: boolean;
  exitCode: number | null;
  success: boolean | null;
  output: string;
};

export const terminalSessions = writable<TerminalSession[]>([]);
export const activeTerminalId = writable<string | null>(null);

export async function createTerminal(
  options: {
    cwd?: string | null;
    rows?: number;
    cols?: number;
    command?: string | null;
    label?: string | null;
  } = {},
) {
  const response = await spawnTerminal(options);
  const session: TerminalSession = {
    id: response.sessionId,
    label: response.label,
    cwd: options.cwd ?? null,
    active: true,
    exited: false,
    exitCode: null,
    success: null,
    output: "",
  };

  terminalSessions.update((sessions) => [
    ...sessions.map((item) => ({ ...item, active: false })),
    session,
  ]);
  activeTerminalId.set(session.id);

  return session;
}

export function setActiveTerminal(sessionId: string) {
  activeTerminalId.set(sessionId);
  terminalSessions.update((sessions) =>
    sessions.map((session) => ({
      ...session,
      active: session.id === sessionId,
    })),
  );
}

export async function sendTerminalInput(sessionId: string, data: string) {
  await writeTerminal(sessionId, data);
}

export async function resizeActiveTerminal(rows: number, cols: number) {
  const sessionId = get(activeTerminalId);

  if (!sessionId) {
    return;
  }

  await resizeTerminal(sessionId, rows, cols);
}

export async function closeTerminal(sessionId: string) {
  await killTerminal(sessionId);
  terminalSessions.update((sessions) =>
    sessions.filter((session) => session.id !== sessionId),
  );

  const remainingSessions = get(terminalSessions);
  activeTerminalId.set(remainingSessions.at(-1)?.id ?? null);
}

export async function registerTerminalEvents() {
  if (!browser) {
    return;
  }

  await onTerminalOutput((event) => {
    terminalSessions.update((sessions) =>
      sessions.map((session) =>
        session.id === event.sessionId
          ? { ...session, output: session.output + event.data }
          : session,
      ),
    );
  });

  await onTerminalExit((event) => {
    markProjectActionTerminalExit(event);
    terminalSessions.update((sessions) =>
      sessions.map((session) =>
        session.id === event.sessionId
          ? {
              ...session,
              exited: true,
              active: false,
              exitCode: event.exitCode,
              success: event.success,
            }
          : session,
      ),
    );
  });
}
