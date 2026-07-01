<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { FitAddon } from "@xterm/addon-fit";
  import { Terminal } from "@xterm/xterm";
  import "@xterm/xterm/css/xterm.css";
  import { resizeTerminal, writeTerminal } from "$lib/services/terminal";

  export let sessionId: string;
  export let output = "";

  let container: HTMLDivElement;
  let terminal: Terminal;
  let fitAddon: FitAddon;
  let resizeObserver: ResizeObserver | null = null;
  let dataDisposable: { dispose: () => void } | null = null;
  let lastSessionId: string | null = null;
  let lastOutputLength = 0;

  $: if (terminal && sessionId !== lastSessionId) {
    terminal.reset();
    terminal.write(output);
    lastSessionId = sessionId;
    lastOutputLength = output.length;
    fit();
  }

  $: if (
    terminal &&
    sessionId === lastSessionId &&
    output.length > lastOutputLength
  ) {
    terminal.write(output.slice(lastOutputLength));
    lastOutputLength = output.length;
  }

  onMount(() => {
    terminal = new Terminal({
      allowProposedApi: false,
      convertEol: true,
      cursorBlink: true,
      fontFamily:
        "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
      fontSize: 13,
      rows: 20,
      theme: {
        background: "#111418",
        foreground: "#d7dde5",
        cursor: "#66c2a5",
        selectionBackground: "#334155",
      },
    });
    fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(container);
    terminal.focus();
    dataDisposable = terminal.onData((data) => {
      writeTerminal(sessionId, data);
    });
    resizeObserver = new ResizeObserver(fit);
    resizeObserver.observe(container);
    terminal.write(output);
    lastSessionId = sessionId;
    lastOutputLength = output.length;
    fit();
  });

  onDestroy(() => {
    dataDisposable?.dispose();
    resizeObserver?.disconnect();
    terminal?.dispose();
  });

  export function focus() {
    terminal?.focus();
  }

  export function fit() {
    if (!terminal || !fitAddon || !container.clientHeight) {
      return;
    }

    fitAddon.fit();
    resizeTerminal(sessionId, terminal.rows, terminal.cols);
  }

  export function copySelection() {
    const selection = terminal?.getSelection();

    if (selection) {
      navigator.clipboard.writeText(selection);
    }
  }

  export async function pasteClipboard() {
    const text = await navigator.clipboard.readText();

    if (text) {
      writeTerminal(sessionId, text);
    }
  }
</script>

<div bind:this={container} class="h-full min-h-0 w-full"></div>
