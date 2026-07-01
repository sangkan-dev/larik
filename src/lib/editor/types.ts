export type MonacoEditorController = {
  disposeModel: (path: string) => void;
  find: () => void;
  replace: () => void;
  formatDocument: () => void;
  goToLine: () => void;
  saveViewState: () => void;
  toggleMinimap: () => void;
  toggleWordWrap: () => void;
};
