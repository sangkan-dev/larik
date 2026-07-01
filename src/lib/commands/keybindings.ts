import defaultKeybindingsJson from "./default-keybindings.json";

export type KeybindingCommandId =
  | "workspace.openFolder"
  | "editor.save"
  | "editor.saveAll"
  | "file.quickOpen"
  | "commandPalette.open"
  | "terminal.toggle"
  | "view.toggleSidebar"
  | "editor.closeTab"
  | "editor.find"
  | "editor.replace"
  | "editor.goToLine"
  | "editor.toggleMinimap"
  | "editor.toggleWordWrap"
  | "editor.formatDocument";

export type Keybinding = {
  command: KeybindingCommandId;
  key: string;
};

export const defaultKeybindings = defaultKeybindingsJson as Keybinding[];

export function keybindingFromEvent(event: KeyboardEvent) {
  const parts = [
    event.ctrlKey || event.metaKey ? "Ctrl" : null,
    event.altKey ? "Alt" : null,
    event.shiftKey ? "Shift" : null,
    event.key.length === 1 ? event.key.toUpperCase() : normalizeKey(event.key),
  ].filter(Boolean);

  return parts.join("+");
}

function normalizeKey(key: string) {
  if (key === " ") return "Space";
  if (key === "Escape") return "Esc";
  return key;
}
