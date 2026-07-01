import type { KeybindingCommandId } from "$lib/commands/keybindings";

export type CommandCategory =
  "Workspace" | "Editor" | "File" | "View" | "Terminal";

export type CommandMetadata = {
  id: KeybindingCommandId;
  title: string;
  category: CommandCategory;
  shortcut?: string;
};

export type AppCommand = CommandMetadata & {
  handler: () => void | Promise<void>;
};

export type CommandRegistry = {
  all: () => AppCommand[];
  execute: (id: KeybindingCommandId) => void | Promise<void>;
  register: (command: AppCommand) => void;
};

export const baseCommandMetadata: CommandMetadata[] = [
  {
    id: "workspace.openFolder",
    title: "Open Folder",
    category: "Workspace",
    shortcut: "Ctrl+O",
  },
  {
    id: "editor.save",
    title: "Save",
    category: "Editor",
    shortcut: "Ctrl+S",
  },
  {
    id: "editor.saveAll",
    title: "Save All",
    category: "Editor",
    shortcut: "Ctrl+Shift+S",
  },
  {
    id: "file.quickOpen",
    title: "Quick Open File",
    category: "File",
    shortcut: "Ctrl+P",
  },
  {
    id: "commandPalette.open",
    title: "Command Palette",
    category: "Workspace",
    shortcut: "Ctrl+Shift+P",
  },
  {
    id: "terminal.toggle",
    title: "Toggle Terminal",
    category: "Terminal",
    shortcut: "Ctrl+`",
  },
  {
    id: "view.toggleSidebar",
    title: "Toggle Sidebar",
    category: "View",
    shortcut: "Ctrl+B",
  },
  {
    id: "editor.closeTab",
    title: "Close Tab",
    category: "Editor",
    shortcut: "Ctrl+W",
  },
  {
    id: "editor.find",
    title: "Find in File",
    category: "Editor",
    shortcut: "Ctrl+F",
  },
  {
    id: "editor.replace",
    title: "Replace in File",
    category: "Editor",
    shortcut: "Ctrl+H",
  },
  {
    id: "editor.goToLine",
    title: "Go to Line",
    category: "Editor",
    shortcut: "Ctrl+G",
  },
  {
    id: "editor.toggleMinimap",
    title: "Toggle Minimap",
    category: "Editor",
    shortcut: "Alt+M",
  },
  {
    id: "editor.toggleWordWrap",
    title: "Toggle Word Wrap",
    category: "Editor",
    shortcut: "Alt+Z",
  },
  {
    id: "editor.formatDocument",
    title: "Format Document",
    category: "Editor",
    shortcut: "Alt+Shift+F",
  },
];

export function createCommandRegistry(commands: AppCommand[]): CommandRegistry {
  const commandMap = new Map<KeybindingCommandId, AppCommand>();

  for (const command of commands) {
    commandMap.set(command.id, command);
  }

  return {
    all() {
      return [...commandMap.values()];
    },
    execute(id) {
      return commandMap.get(id)?.handler();
    },
    register(command) {
      commandMap.set(command.id, command);
    },
  };
}
