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

  export let content = "";
  export let path: string;
  export let theme: "larik-dark" | "larik-light" = "larik-dark";
  export let minimap = false;
  export let wordWrap = false;
  export let onChange: (content: string) => void;
  export let onReady: (controller: MonacoEditorController) => void;

  let container: HTMLDivElement;
  let editor: monaco.editor.IStandaloneCodeEditor | null = null;
  let model: monaco.editor.ITextModel | null = null;
  let changeSubscription: monaco.IDisposable | null = null;
  let resizeObserver: ResizeObserver | null = null;
  let currentModelPath: string | null = null;
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

  onMount(() => {
    defineThemes();
    editor = monaco.editor.create(container, {
      automaticLayout: false,
      fontFamily:
        "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
      fontSize: 13,
      lineHeight: 22,
      minimap: { enabled: minimap },
      padding: { top: 12, bottom: 12 },
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

  onDestroy(() => {
    changeSubscription?.dispose();
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
    const viewState = viewStates.get(nextPath);
    if (viewState) {
      editor.restoreViewState(viewState);
    }
    editor.focus();
    changeSubscription?.dispose();
    changeSubscription = model.onDidChangeContent(() => {
      onChange(model?.getValue() ?? "");
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
</script>

<div bind:this={container} class="min-h-0 flex-1"></div>
