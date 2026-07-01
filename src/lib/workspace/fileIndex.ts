import type { FileTreeEntry } from "$lib/services/workspace";

export type WorkspaceFileIndexItem = {
  id: string;
  title: string;
  subtitle: string;
  path: string;
};

export function indexWorkspaceFiles(entries: FileTreeEntry[]) {
  const files: WorkspaceFileIndexItem[] = [];

  walkFileTree(entries, files);

  return files.sort(
    (left, right) =>
      left.subtitle.localeCompare(right.subtitle) ||
      left.title.localeCompare(right.title),
  );
}

function walkFileTree(
  entries: FileTreeEntry[],
  files: WorkspaceFileIndexItem[],
) {
  for (const entry of entries) {
    if (entry.isDir) {
      walkFileTree(entry.children ?? [], files);
      continue;
    }

    files.push({
      id: entry.path,
      title: entry.name,
      subtitle: entry.path,
      path: entry.path,
    });
  }
}
