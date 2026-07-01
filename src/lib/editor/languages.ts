export function languageFromPath(path: string) {
  const fileName = path.split(/[\\/]/).at(-1)?.toLowerCase() ?? "";
  const extension = fileName.split(".").at(-1) ?? "";

  if (fileName === "dockerfile") return "dockerfile";
  if (fileName === "cargo.toml" || fileName.endsWith(".toml")) return "toml";
  if (fileName.endsWith(".svelte")) return "html";

  const languages: Record<string, string> = {
    cjs: "javascript",
    css: "css",
    go: "go",
    html: "html",
    js: "javascript",
    json: "json",
    jsx: "javascript",
    md: "markdown",
    php: "php",
    rs: "rust",
    sh: "shell",
    sql: "sql",
    ts: "typescript",
    tsx: "typescript",
    txt: "plaintext",
    yaml: "yaml",
    yml: "yaml",
  };

  return languages[extension] ?? "plaintext";
}
