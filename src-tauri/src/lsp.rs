use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tauri::{AppHandle, Emitter, State};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(8);

type PendingRequests = Arc<Mutex<HashMap<u64, mpsc::Sender<Result<Value, String>>>>>;

#[derive(Default)]
pub struct LspState(Mutex<HashMap<String, LspServerHandle>>);

struct LspServerHandle {
    child: Child,
    stdin: Arc<Mutex<std::process::ChildStdin>>,
    pending: PendingRequests,
    next_id: AtomicU64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspStartRequest {
    pub root_path: String,
    pub language_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspLanguageRequest {
    pub root_path: String,
    pub language_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspDocumentRequest {
    pub root_path: String,
    pub language_id: String,
    pub path: String,
    pub content: Option<String>,
    pub version: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspPositionRequest {
    pub root_path: String,
    pub language_id: String,
    pub path: String,
    pub line: u32,
    pub character: u32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LspServerInfo {
    pub language_id: String,
    pub running: bool,
    pub command: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LspDiagnosticsEvent {
    pub language_id: String,
    pub uri: String,
    pub path: Option<String>,
    pub diagnostics: Vec<Value>,
}

#[derive(Clone)]
struct LanguageServerConfig {
    language_id: &'static str,
    command: &'static str,
    args: &'static [&'static str],
}

#[tauri::command]
pub fn lsp_start(
    app: AppHandle,
    state: State<LspState>,
    request: LspStartRequest,
) -> Result<LspServerInfo, String> {
    let root = canonical_dir(&request.root_path)?;
    let config = language_config(&request.language_id)?;
    let key = server_key(&root, config.language_id);

    if state
        .0
        .lock()
        .map_err(|error| error.to_string())?
        .contains_key(&key)
    {
        return Ok(LspServerInfo {
            language_id: config.language_id.to_string(),
            running: true,
            command: command_label(&config),
        });
    }

    let mut child = Command::new(config.command)
        .args(config.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(&root)
        .spawn()
        .map_err(|error| {
            format!(
                "Cannot start {}. Install the language server and ensure it is on PATH. {error}",
                command_label(&config)
            )
        })?;

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Language server stdin is unavailable".to_string())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Language server stdout is unavailable".to_string())?;
    let stderr = child.stderr.take();
    let pending = Arc::new(Mutex::new(HashMap::new()));
    let stdin = Arc::new(Mutex::new(stdin));
    spawn_stdout_reader(
        app,
        config.language_id.to_string(),
        stdout,
        Arc::clone(&pending),
    );
    if let Some(stderr) = stderr {
        spawn_stderr_reader(stderr);
    }

    let handle = LspServerHandle {
        child,
        stdin: Arc::clone(&stdin),
        pending: Arc::clone(&pending),
        next_id: AtomicU64::new(1),
    };

    send_request_to_handle(
        &handle,
        "initialize",
        json!({
            "processId": std::process::id(),
            "rootUri": path_to_uri(&root),
            "workspaceFolders": [{
                "uri": path_to_uri(&root),
                "name": root.file_name().and_then(|name| name.to_str()).unwrap_or("workspace")
            }],
            "capabilities": client_capabilities()
        }),
    )?;
    send_notification_to_stdin(&stdin, "initialized", json!({}))?;

    state
        .0
        .lock()
        .map_err(|error| error.to_string())?
        .insert(key, handle);

    Ok(LspServerInfo {
        language_id: config.language_id.to_string(),
        running: true,
        command: command_label(&config),
    })
}

#[tauri::command]
pub fn lsp_stop(state: State<LspState>, request: LspLanguageRequest) -> Result<(), String> {
    let root = canonical_dir(&request.root_path)?;
    let key = server_key(&root, &request.language_id);
    let mut handle = state
        .0
        .lock()
        .map_err(|error| error.to_string())?
        .remove(&key);

    if let Some(mut handle) = handle.take() {
        let _ = send_request_to_handle(&handle, "shutdown", json!(null));
        let _ = send_notification_to_stdin(&handle.stdin, "exit", json!(null));
        let _ = handle.child.kill();
    }

    Ok(())
}

#[tauri::command]
pub fn lsp_document_open(
    state: State<LspState>,
    request: LspDocumentRequest,
) -> Result<(), String> {
    let root = canonical_dir(&request.root_path)?;
    let path = canonical_child(&root, &request.path)?;
    let content = request.content.unwrap_or_default();
    send_notification(
        &state,
        &root,
        &request.language_id,
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": path_to_uri(&path),
                "languageId": request.language_id,
                "version": request.version.unwrap_or(1),
                "text": content
            }
        }),
    )
}

#[tauri::command]
pub fn lsp_document_change(
    state: State<LspState>,
    request: LspDocumentRequest,
) -> Result<(), String> {
    let root = canonical_dir(&request.root_path)?;
    let path = canonical_child(&root, &request.path)?;
    let content = request.content.unwrap_or_default();
    send_notification(
        &state,
        &root,
        &request.language_id,
        "textDocument/didChange",
        json!({
            "textDocument": {
                "uri": path_to_uri(&path),
                "version": request.version.unwrap_or(1)
            },
            "contentChanges": [{ "text": content }]
        }),
    )
}

#[tauri::command]
pub fn lsp_document_save(
    state: State<LspState>,
    request: LspDocumentRequest,
) -> Result<(), String> {
    let root = canonical_dir(&request.root_path)?;
    let path = canonical_child(&root, &request.path)?;
    send_notification(
        &state,
        &root,
        &request.language_id,
        "textDocument/didSave",
        json!({
            "textDocument": { "uri": path_to_uri(&path) },
            "text": request.content.unwrap_or_default()
        }),
    )
}

#[tauri::command]
pub fn lsp_document_close(
    state: State<LspState>,
    request: LspDocumentRequest,
) -> Result<(), String> {
    let root = canonical_dir(&request.root_path)?;
    let path = canonical_child(&root, &request.path)?;
    send_notification(
        &state,
        &root,
        &request.language_id,
        "textDocument/didClose",
        json!({
            "textDocument": { "uri": path_to_uri(&path) }
        }),
    )
}

#[tauri::command]
pub fn lsp_completion(
    state: State<LspState>,
    request: LspPositionRequest,
) -> Result<Value, String> {
    position_request(&state, request, "textDocument/completion")
}

#[tauri::command]
pub fn lsp_hover(state: State<LspState>, request: LspPositionRequest) -> Result<Value, String> {
    position_request(&state, request, "textDocument/hover")
}

#[tauri::command]
pub fn lsp_definition(
    state: State<LspState>,
    request: LspPositionRequest,
) -> Result<Value, String> {
    position_request(&state, request, "textDocument/definition")
}

#[tauri::command]
pub fn lsp_formatting(
    state: State<LspState>,
    request: LspDocumentRequest,
) -> Result<Value, String> {
    let root = canonical_dir(&request.root_path)?;
    let path = canonical_child(&root, &request.path)?;
    send_request(
        &state,
        &root,
        &request.language_id,
        "textDocument/formatting",
        json!({
            "textDocument": { "uri": path_to_uri(&path) },
            "options": { "tabSize": 2, "insertSpaces": true }
        }),
    )
}

fn position_request(
    state: &State<LspState>,
    request: LspPositionRequest,
    method: &str,
) -> Result<Value, String> {
    let root = canonical_dir(&request.root_path)?;
    let path = canonical_child(&root, &request.path)?;
    send_request(
        state,
        &root,
        &request.language_id,
        method,
        json!({
            "textDocument": { "uri": path_to_uri(&path) },
            "position": {
                "line": request.line,
                "character": request.character
            }
        }),
    )
}

fn send_request(
    state: &State<LspState>,
    root: &Path,
    language_id: &str,
    method: &str,
    params: Value,
) -> Result<Value, String> {
    let key = server_key(root, language_id);
    let (id, stdin, pending) = {
        let servers = state.0.lock().map_err(|error| error.to_string())?;
        let handle = servers
            .get(&key)
            .ok_or_else(|| "Language server is not running".to_string())?;
        (
            handle.next_id.fetch_add(1, Ordering::Relaxed),
            Arc::clone(&handle.stdin),
            Arc::clone(&handle.pending),
        )
    };

    send_request_with_connection(id, stdin, pending, method, params)
}

fn send_request_to_handle(
    handle: &LspServerHandle,
    method: &str,
    params: Value,
) -> Result<Value, String> {
    let id = handle.next_id.fetch_add(1, Ordering::Relaxed);
    send_request_with_connection(
        id,
        Arc::clone(&handle.stdin),
        Arc::clone(&handle.pending),
        method,
        params,
    )
}

fn send_request_with_connection(
    id: u64,
    stdin: Arc<Mutex<std::process::ChildStdin>>,
    pending: PendingRequests,
    method: &str,
    params: Value,
) -> Result<Value, String> {
    let (sender, receiver) = mpsc::channel();
    pending
        .lock()
        .map_err(|error| error.to_string())?
        .insert(id, sender);
    let message = json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params
    });

    if let Err(error) = write_json_rpc(&stdin, &message) {
        let _ = pending
            .lock()
            .map_err(|lock_error| lock_error.to_string())?
            .remove(&id);
        return Err(error);
    }

    receiver
        .recv_timeout(REQUEST_TIMEOUT)
        .map_err(|_| format!("LSP request timed out: {method}"))?
}

fn send_notification(
    state: &State<LspState>,
    root: &Path,
    language_id: &str,
    method: &str,
    params: Value,
) -> Result<(), String> {
    let key = server_key(root, language_id);
    let stdin = {
        let servers = state.0.lock().map_err(|error| error.to_string())?;
        let handle = servers
            .get(&key)
            .ok_or_else(|| "Language server is not running".to_string())?;
        Arc::clone(&handle.stdin)
    };

    send_notification_to_stdin(&stdin, method, params)
}

fn send_notification_to_stdin(
    stdin: &Arc<Mutex<std::process::ChildStdin>>,
    method: &str,
    params: Value,
) -> Result<(), String> {
    write_json_rpc(
        stdin,
        &json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        }),
    )
}

fn write_json_rpc(
    stdin: &Arc<Mutex<std::process::ChildStdin>>,
    value: &Value,
) -> Result<(), String> {
    let body = serde_json::to_string(value).map_err(|error| error.to_string())?;
    let mut writer = stdin.lock().map_err(|error| error.to_string())?;
    writer
        .write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())
        .map_err(|error| error.to_string())?;
    writer
        .write_all(body.as_bytes())
        .map_err(|error| error.to_string())?;
    writer.flush().map_err(|error| error.to_string())
}

fn spawn_stdout_reader(
    app: AppHandle,
    language_id: String,
    stdout: ChildStdout,
    pending: PendingRequests,
) {
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        while let Ok(Some(message)) = read_json_rpc(&mut reader) {
            if let Some(id) = message.get("id").and_then(Value::as_u64) {
                let result = if let Some(error) = message.get("error") {
                    Err(error.to_string())
                } else {
                    Ok(message.get("result").cloned().unwrap_or(Value::Null))
                };
                if let Ok(mut pending) = pending.lock() {
                    if let Some(sender) = pending.remove(&id) {
                        let _ = sender.send(result);
                    }
                }
                continue;
            }

            if message.get("method").and_then(Value::as_str)
                == Some("textDocument/publishDiagnostics")
            {
                if let Some(params) = message.get("params") {
                    let uri = params
                        .get("uri")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    let diagnostics = params
                        .get("diagnostics")
                        .and_then(Value::as_array)
                        .cloned()
                        .unwrap_or_default();
                    let _ = app.emit(
                        "lsp://diagnostics",
                        LspDiagnosticsEvent {
                            language_id: language_id.clone(),
                            path: uri_to_path(&uri),
                            uri,
                            diagnostics,
                        },
                    );
                }
            }
        }
    });
}

fn spawn_stderr_reader(stderr: ChildStderr) {
    thread::spawn(move || {
        let mut reader = BufReader::new(stderr);
        let mut buffer = String::new();
        while reader.read_line(&mut buffer).unwrap_or(0) > 0 {
            buffer.clear();
        }
    });
}

fn read_json_rpc(reader: &mut BufReader<ChildStdout>) -> Result<Option<Value>, String> {
    let mut content_length = None;
    loop {
        let mut line = String::new();
        let read = reader
            .read_line(&mut line)
            .map_err(|error| error.to_string())?;
        if read == 0 {
            return Ok(None);
        }
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break;
        }
        if let Some(length) = trimmed.strip_prefix("Content-Length:") {
            content_length = Some(
                length
                    .trim()
                    .parse::<usize>()
                    .map_err(|error| error.to_string())?,
            );
        }
    }

    let length = content_length.ok_or_else(|| "Missing LSP Content-Length".to_string())?;
    let mut body = vec![0; length];
    reader
        .read_exact(&mut body)
        .map_err(|error| error.to_string())?;
    serde_json::from_slice(&body)
        .map(Some)
        .map_err(|error| error.to_string())
}

fn language_config(language_id: &str) -> Result<LanguageServerConfig, String> {
    match language_id {
        "typescript" | "javascript" => Ok(LanguageServerConfig {
            language_id: if language_id == "javascript" {
                "javascript"
            } else {
                "typescript"
            },
            command: "typescript-language-server",
            args: &["--stdio"],
        }),
        "svelte" => Ok(LanguageServerConfig {
            language_id: "svelte",
            command: "svelteserver",
            args: &["--stdio"],
        }),
        "rust" => Ok(LanguageServerConfig {
            language_id: "rust",
            command: "rust-analyzer",
            args: &[],
        }),
        "go" => Ok(LanguageServerConfig {
            language_id: "go",
            command: "gopls",
            args: &["serve"],
        }),
        "php" => Ok(LanguageServerConfig {
            language_id: "php",
            command: "intelephense",
            args: &["--stdio"],
        }),
        _ => Err("No language server config for this language".to_string()),
    }
}

fn client_capabilities() -> Value {
    json!({
        "textDocument": {
            "synchronization": {
                "didSave": true,
                "dynamicRegistration": false
            },
            "completion": {
                "completionItem": {
                    "snippetSupport": false,
                    "documentationFormat": ["markdown", "plaintext"]
                }
            },
            "hover": {
                "contentFormat": ["markdown", "plaintext"]
            },
            "definition": {},
            "formatting": {}
        },
        "workspace": {
            "configuration": false,
            "workspaceFolders": true
        }
    })
}

fn canonical_dir(path: &str) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path).map_err(|error| error.to_string())?;
    if canonical.is_dir() {
        Ok(canonical)
    } else {
        Err("Workspace path is not a folder".to_string())
    }
}

fn canonical_child(root: &Path, path: &str) -> Result<PathBuf, String> {
    let candidate = PathBuf::from(path);
    let canonical = fs::canonicalize(candidate).map_err(|error| error.to_string())?;
    if canonical.starts_with(root) {
        Ok(canonical)
    } else {
        Err("Path is outside the active workspace".to_string())
    }
}

fn server_key(root: &Path, language_id: &str) -> String {
    format!("{}::{language_id}", root.to_string_lossy())
}

fn command_label(config: &LanguageServerConfig) -> String {
    if config.args.is_empty() {
        config.command.to_string()
    } else {
        format!("{} {}", config.command, config.args.join(" "))
    }
}

fn path_to_uri(path: &Path) -> String {
    format!("file://{}", path.to_string_lossy())
}

fn uri_to_path(uri: &str) -> Option<String> {
    uri.strip_prefix("file://").map(ToString::to_string)
}
