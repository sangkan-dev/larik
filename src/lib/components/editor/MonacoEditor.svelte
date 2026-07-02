<script lang="ts" context="module">
  import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
  import CssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
  import HtmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
  import JsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
  import TsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

  (
    globalThis as typeof globalThis & {
      MonacoEnvironment: {
        getWorker: (workerId: string, label: string) => Worker;
      };
    }
  ).MonacoEnvironment = {
    getWorker(_workerId: string, label: string) {
      if (label === "json") return new JsonWorker();
      if (label === "css" || label === "scss" || label === "less") {
        return new CssWorker();
      }
      if (label === "html" || label === "handlebars" || label === "razor") {
        return new HtmlWorker();
      }
      if (label === "typescript" || label === "javascript") {
        return new TsWorker();
      }

      return new EditorWorker();
    },
  };
</script>

<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import * as monaco from "monaco-editor/esm/vs/editor/editor.api.js";
  import "monaco-editor/esm/vs/basic-languages/css/css.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/dockerfile/dockerfile.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/go/go.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/html/html.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/javascript/javascript.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/markdown/markdown.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/php/php.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/rust/rust.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/shell/shell.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/sql/sql.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/typescript/typescript.contribution.js";
  import "monaco-editor/esm/vs/basic-languages/yaml/yaml.contribution.js";
  import "monaco-editor/esm/vs/language/css/monaco.contribution.js";
  import "monaco-editor/esm/vs/language/html/monaco.contribution.js";
  import "monaco-editor/esm/vs/language/json/monaco.contribution.js";
  import "monaco-editor/esm/vs/language/typescript/monaco.contribution.js";
  import "monaco-editor/min/vs/editor/editor.main.css";
  import { languageFromPath } from "$lib/editor/languages";
  import type { MonacoEditorController } from "$lib/editor/types";
  import {
    lspLanguageFromPath,
    requestLspCompletion,
    requestLspDefinition,
    requestLspFormatting,
    requestLspHover,
    type LspDiagnostic,
  } from "$lib/services/lsp";

  export let content = "";
  export let path: string;
  export let theme: "larik-dark" | "larik-light" = "larik-dark";
  export let minimap = false;
  export let wordWrap = false;
  export let gitDiff: string | null = null;
  export let workspaceRoot: string | null = null;
  export let diagnostics: LspDiagnostic[] = [];
  export let onChange: (content: string) => void;
  export let onCursorChange: (position: {
    lineNumber: number;
    column: number;
  }) => void;
  export let onReady: (controller: MonacoEditorController) => void;

  let container: HTMLDivElement;
  let editor: monaco.editor.IStandaloneCodeEditor | null = null;
  let model: monaco.editor.ITextModel | null = null;
  let gitDecorationCollection: monaco.editor.IEditorDecorationsCollection | null =
    null;
  let changeSubscription: monaco.IDisposable | null = null;
  let cursorSubscription: monaco.IDisposable | null = null;
  let resizeObserver: ResizeObserver | null = null;
  let currentModelPath: string | null = null;
  let lspProvidersRegistered = false;
  const viewStates = new Map<
    string,
    monaco.editor.ICodeEditorViewState | null
  >();

  $: if (editor && path) {
    attachModel(path, content);
  }

  $: if (editor && theme) {
    monaco.editor.setTheme(theme);
  }

  $: if (editor) {
    editor.updateOptions({
      minimap: { enabled: minimap },
      wordWrap: wordWrap ? "on" : "off",
    });
  }

  $: if (model && content !== model.getValue()) {
    model.setValue(content);
  }

  $: if (model) {
    applyDiagnostics(diagnostics);
  }

  onMount(() => {
    defineThemes();
    registerLspProviders();
    editor = monaco.editor.create(container, {
      automaticLayout: false,
      fontFamily:
        "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
      fontSize: 13,
      lineHeight: 22,
      minimap: { enabled: minimap },
      padding: { top: 12, bottom: 12 },
      glyphMargin: true,
      renderLineHighlight: "line",
      scrollBeyondLastLine: false,
      tabSize: 2,
      theme,
      wordWrap: wordWrap ? "on" : "off",
    });
    resizeObserver = new ResizeObserver(() => editor?.layout());
    resizeObserver.observe(container);
    attachModel(path, content);
    onReady(createController());
  });

  $: if (editor && gitDecorationCollection) {
    applyGitDecorations(gitDiff);
  }

  onDestroy(() => {
    changeSubscription?.dispose();
    cursorSubscription?.dispose();
    if (model && currentModelPath) {
      viewStates.set(currentModelPath, editor?.saveViewState() ?? null);
    }
    resizeObserver?.disconnect();
    editor?.dispose();
  });

  function defineThemes() {
    monaco.editor.defineTheme("larik-dark", {
      base: "vs-dark",
      inherit: true,
      rules: [],
      colors: {
        "editor.background": "#111418",
        "editor.foreground": "#d7dde5",
        "editor.lineHighlightBackground": "#1b2128",
        "editorCursor.foreground": "#66c2a5",
        "editorLineNumber.foreground": "#6f7a86",
      },
    });
    monaco.editor.defineTheme("larik-light", {
      base: "vs",
      inherit: true,
      rules: [],
      colors: {
        "editor.background": "#f8fafc",
        "editor.foreground": "#1f2937",
        "editor.lineHighlightBackground": "#eef2f7",
        "editorCursor.foreground": "#0f8a6a",
        "editorLineNumber.foreground": "#8b96a3",
      },
    });
  }

  function attachModel(nextPath: string, nextContent: string) {
    if (!editor || !nextPath) {
      return;
    }

    if (model && currentModelPath) {
      viewStates.set(currentModelPath, editor.saveViewState());
    }

    const uri = monaco.Uri.file(nextPath);
    const language = languageFromPath(nextPath);
    const existingModel = monaco.editor.getModel(uri);
    model =
      existingModel ??
      monaco.editor.createModel(
        nextContent,
        language,
        monaco.Uri.file(nextPath),
      );

    monaco.editor.setModelLanguage(model, language);
    editor.setModel(model);
    currentModelPath = nextPath;
    applyGitDecorations(gitDiff);
    applyDiagnostics(diagnostics);
    const viewState = viewStates.get(nextPath);
    if (viewState) {
      editor.restoreViewState(viewState);
    }
    editor.focus();
    changeSubscription?.dispose();
    changeSubscription = model.onDidChangeContent(() => {
      onChange(model?.getValue() ?? "");
    });
    cursorSubscription?.dispose();
    cursorSubscription = editor.onDidChangeCursorPosition((event) => {
      onCursorChange(event.position);
    });
  }

  function createController(): MonacoEditorController {
    return {
      disposeModel(disposedPath) {
        const uri = monaco.Uri.file(disposedPath);
        monaco.editor.getModel(uri)?.dispose();
        viewStates.delete(disposedPath);
      },
      find() {
        editor?.trigger("keyboard", "actions.find", null);
      },
      replace() {
        editor?.trigger(
          "keyboard",
          "editor.action.startFindReplaceAction",
          null,
        );
      },
      formatDocument() {
        editor?.trigger("keyboard", "editor.action.formatDocument", null);
      },
      goToLine() {
        editor?.trigger("keyboard", "editor.action.gotoLine", null);
      },
      saveViewState() {
        if (model && currentModelPath) {
          viewStates.set(currentModelPath, editor?.saveViewState() ?? null);
        }
      },
      toggleMinimap() {
        editor?.updateOptions({ minimap: { enabled: !minimap } });
      },
      toggleWordWrap() {
        editor?.updateOptions({ wordWrap: wordWrap ? "off" : "on" });
      },
    };
  }

  function applyGitDecorations(diff: string | null) {
    if (!editor || !model) {
      return;
    }

    gitDecorationCollection ??= editor.createDecorationsCollection();
    gitDecorationCollection.set(
      parseChangedLines(diff).map((lineNumber) => ({
        range: new monaco.Range(lineNumber, 1, lineNumber, 1),
        options: {
          isWholeLine: true,
          className: "larik-git-line",
          glyphMarginClassName: "larik-git-glyph",
          glyphMarginHoverMessage: { value: "Changed in Git working tree" },
        },
      })),
    );
  }

  function parseChangedLines(diff: string | null) {
    if (!diff) {
      return [];
    }

    const changedLines = new Set<number>();
    let nextLine = 0;

    for (const line of diff.split("\n")) {
      const hunk = /^@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@/.exec(line);
      if (hunk) {
        nextLine = Number(hunk[1]);
        continue;
      }
      if (nextLine === 0 || line.startsWith("\\ No newline")) {
        continue;
      }
      if (line.startsWith("+")) {
        changedLines.add(nextLine);
        nextLine += 1;
        continue;
      }
      if (line.startsWith("-")) {
        changedLines.add(Math.max(nextLine, 1));
        continue;
      }
      if (line.startsWith(" ")) {
        nextLine += 1;
      }
    }

    return [...changedLines];
  }

  function applyDiagnostics(nextDiagnostics: LspDiagnostic[]) {
    if (!model) {
      return;
    }

    monaco.editor.setModelMarkers(
      model,
      "larik-lsp",
      nextDiagnostics.map((diagnostic) => ({
        startLineNumber: diagnostic.range.start.line + 1,
        startColumn: diagnostic.range.start.character + 1,
        endLineNumber: diagnostic.range.end.line + 1,
        endColumn: diagnostic.range.end.character + 1,
        message: diagnostic.message,
        source: diagnostic.source ?? "LSP",
        severity: diagnosticSeverity(diagnostic.severity),
      })),
    );
  }

  function diagnosticSeverity(severity: number | undefined) {
    if (severity === 1) return monaco.MarkerSeverity.Error;
    if (severity === 2) return monaco.MarkerSeverity.Warning;
    if (severity === 3) return monaco.MarkerSeverity.Info;
    return monaco.MarkerSeverity.Hint;
  }

  function registerLspProviders() {
    if (lspProvidersRegistered) {
      return;
    }
    lspProvidersRegistered = true;

    const languages = ["typescript", "javascript", "html", "rust", "go", "php"];
    for (const language of languages) {
      monaco.languages.registerCompletionItemProvider(language, {
        triggerCharacters: [".", '"', "'", "/", "@", "<"],
        async provideCompletionItems(providerModel, position) {
          const context = lspContext(providerModel);
          if (!context) {
            return { suggestions: [] };
          }

          const result = await requestLspCompletion({
            ...context,
            line: position.lineNumber - 1,
            character: position.column - 1,
          });

          const word = providerModel.getWordUntilPosition(position);
          const range = new monaco.Range(
            position.lineNumber,
            word.startColumn,
            position.lineNumber,
            word.endColumn,
          );

          return {
            suggestions: completionItems(result).map((item) => ({
              label: completionLabel(item),
              kind: completionKind(item.kind),
              detail: item.detail,
              documentation: completionDocumentation(item.documentation),
              insertText: item.insertText ?? completionLabel(item),
              range,
            })),
          };
        },
      });

      monaco.languages.registerHoverProvider(language, {
        async provideHover(providerModel, position) {
          const context = lspContext(providerModel);
          if (!context) {
            return null;
          }

          const result = await requestLspHover({
            ...context,
            line: position.lineNumber - 1,
            character: position.column - 1,
          });
          const contents = hoverContents(result);
          return contents.length > 0 ? { contents } : null;
        },
      });

      monaco.languages.registerDefinitionProvider(language, {
        async provideDefinition(providerModel, position) {
          const context = lspContext(providerModel);
          if (!context) {
            return null;
          }

          const result = await requestLspDefinition({
            ...context,
            line: position.lineNumber - 1,
            character: position.column - 1,
          });

          return definitionLocations(result);
        },
      });

      monaco.languages.registerDocumentFormattingEditProvider(language, {
        async provideDocumentFormattingEdits(providerModel) {
          const context = lspContext(providerModel);
          if (!context) {
            return [];
          }

          const result = await requestLspFormatting(
            context.rootPath,
            context.languageId,
            context.path,
          );

          return textEdits(result);
        },
      });
    }
  }

  function lspContext(providerModel: monaco.editor.ITextModel) {
    const modelPath = providerModel.uri.fsPath;
    const languageId = lspLanguageFromPath(modelPath);
    if (!workspaceRoot || !languageId) {
      return null;
    }

    return {
      rootPath: workspaceRoot,
      languageId,
      path: modelPath,
    };
  }

  type RawCompletionItem = {
    label?: string | { label?: string };
    kind?: number;
    detail?: string;
    documentation?: string | { value?: string };
    insertText?: string;
  };

  function completionItems(result: unknown): RawCompletionItem[] {
    if (Array.isArray(result)) {
      return result as RawCompletionItem[];
    }
    if (
      result &&
      typeof result === "object" &&
      "items" in result &&
      Array.isArray((result as { items: unknown }).items)
    ) {
      return (result as { items: RawCompletionItem[] }).items;
    }

    return [];
  }

  function completionLabel(item: RawCompletionItem) {
    if (typeof item.label === "string") {
      return item.label;
    }

    return item.label?.label ?? item.insertText ?? "";
  }

  function completionDocumentation(
    documentation: RawCompletionItem["documentation"],
  ) {
    if (!documentation) {
      return undefined;
    }
    if (typeof documentation === "string") {
      return documentation;
    }

    return { value: documentation.value ?? "" };
  }

  function completionKind(kind: number | undefined) {
    if (!kind) {
      return monaco.languages.CompletionItemKind.Text;
    }

    const map: Record<number, monaco.languages.CompletionItemKind> = {
      1: monaco.languages.CompletionItemKind.Text,
      2: monaco.languages.CompletionItemKind.Method,
      3: monaco.languages.CompletionItemKind.Function,
      4: monaco.languages.CompletionItemKind.Constructor,
      5: monaco.languages.CompletionItemKind.Field,
      6: monaco.languages.CompletionItemKind.Variable,
      7: monaco.languages.CompletionItemKind.Class,
      8: monaco.languages.CompletionItemKind.Interface,
      9: monaco.languages.CompletionItemKind.Module,
      10: monaco.languages.CompletionItemKind.Property,
      14: monaco.languages.CompletionItemKind.Keyword,
      15: monaco.languages.CompletionItemKind.Snippet,
      17: monaco.languages.CompletionItemKind.File,
      18: monaco.languages.CompletionItemKind.Reference,
    };

    return map[kind] ?? monaco.languages.CompletionItemKind.Text;
  }

  function hoverContents(result: unknown) {
    if (!result || typeof result !== "object" || !("contents" in result)) {
      return [];
    }

    const contents = (result as { contents: unknown }).contents;
    const items = Array.isArray(contents) ? contents : [contents];
    return items
      .map((item) => {
        if (typeof item === "string") {
          return { value: item };
        }
        if (item && typeof item === "object" && "value" in item) {
          return { value: String((item as { value: unknown }).value) };
        }
        return null;
      })
      .filter((item): item is { value: string } => item !== null);
  }

  function definitionLocations(result: unknown) {
    const items = Array.isArray(result) ? result : result ? [result] : [];
    const locations: monaco.languages.Location[] = [];
    for (const item of items) {
      if (!item || typeof item !== "object") {
        continue;
      }
      const location = item as { uri?: string; range?: LspDiagnostic["range"] };
      if (!location.uri || !location.range) {
        continue;
      }

      locations.push({
        uri: monaco.Uri.parse(location.uri),
        range: toMonacoRange(location.range),
      });
    }

    return locations;
  }

  function textEdits(result: unknown) {
    if (!Array.isArray(result)) {
      return [];
    }

    const edits: monaco.languages.TextEdit[] = [];
    for (const item of result) {
      if (!item || typeof item !== "object") {
        continue;
      }
      const edit = item as { range?: LspDiagnostic["range"]; newText?: string };
      if (!edit.range) {
        continue;
      }
      edits.push({
        range: toMonacoRange(edit.range),
        text: edit.newText ?? "",
      });
    }

    return edits;
  }

  function toMonacoRange(range: LspDiagnostic["range"]) {
    return new monaco.Range(
      range.start.line + 1,
      range.start.character + 1,
      range.end.line + 1,
      range.end.character + 1,
    );
  }
</script>

<div bind:this={container} class="min-h-0 flex-1"></div>

<style>
  :global(.larik-git-line) {
    background: color-mix(in srgb, var(--success) 12%, transparent);
  }

  :global(.larik-git-glyph) {
    background: var(--success);
    margin-left: 4px;
    width: 3px !important;
  }
</style>
