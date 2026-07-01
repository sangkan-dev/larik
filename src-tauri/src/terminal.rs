use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
};
use tauri::{AppHandle, Emitter};

static NEXT_TERMINAL_ID: AtomicU64 = AtomicU64::new(1);

pub struct TerminalState(pub Mutex<HashMap<String, TerminalSession>>);

pub struct TerminalSession {
    child: Box<dyn Child + Send + Sync>,
    master: Box<dyn MasterPty + Send>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnTerminalRequest {
    pub cwd: Option<String>,
    pub rows: Option<u16>,
    pub cols: Option<u16>,
    pub command: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizeTerminalRequest {
    pub session_id: String,
    pub rows: u16,
    pub cols: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteTerminalRequest {
    pub session_id: String,
    pub data: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TerminalOutput {
    pub session_id: String,
    pub data: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TerminalExit {
    pub session_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnTerminalResponse {
    pub session_id: String,
    pub label: String,
}

#[tauri::command]
pub fn terminal_spawn(
    app: AppHandle,
    state: tauri::State<TerminalState>,
    request: SpawnTerminalRequest,
) -> Result<SpawnTerminalResponse, String> {
    let cwd = resolve_cwd(request.cwd.as_deref())?;
    let rows = request.rows.unwrap_or(24).max(1);
    let cols = request.cols.unwrap_or(80).max(1);
    let session_id = format!("term-{}", NEXT_TERMINAL_ID.fetch_add(1, Ordering::Relaxed));
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|error| error.to_string())?;
    let shell = default_shell();
    let mut command = CommandBuilder::new(&shell);
    command.cwd(cwd);

    let child = pair
        .slave
        .spawn_command(command)
        .map_err(|error| error.to_string())?;
    let reader = pair
        .master
        .try_clone_reader()
        .map_err(|error| error.to_string())?;
    let writer = pair
        .master
        .take_writer()
        .map_err(|error| error.to_string())?;
    let writer = Arc::new(Mutex::new(writer));

    stream_terminal_output(app.clone(), session_id.clone(), reader);

    if let Some(initial_command) = request.command.as_deref() {
        let mut locked_writer = writer.lock().map_err(|error| error.to_string())?;
        locked_writer
            .write_all(format!("{initial_command}\n").as_bytes())
            .map_err(|error| error.to_string())?;
        locked_writer.flush().map_err(|error| error.to_string())?;
    }

    let label = request.label.unwrap_or_else(|| {
        PathBuf::from(&shell)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("terminal")
            .to_string()
    });

    let session = TerminalSession {
        child,
        master: pair.master,
        writer,
    };
    let mut sessions = state.0.lock().map_err(|error| error.to_string())?;
    sessions.insert(session_id.clone(), session);

    Ok(SpawnTerminalResponse { session_id, label })
}

#[tauri::command]
pub fn terminal_write(
    state: tauri::State<TerminalState>,
    request: WriteTerminalRequest,
) -> Result<(), String> {
    let sessions = state.0.lock().map_err(|error| error.to_string())?;
    let session = sessions
        .get(&request.session_id)
        .ok_or_else(|| "Terminal session not found".to_string())?;
    let mut writer = session.writer.lock().map_err(|error| error.to_string())?;

    writer
        .write_all(request.data.as_bytes())
        .map_err(|error| error.to_string())?;
    writer.flush().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn terminal_resize(
    state: tauri::State<TerminalState>,
    request: ResizeTerminalRequest,
) -> Result<(), String> {
    let sessions = state.0.lock().map_err(|error| error.to_string())?;
    let session = sessions
        .get(&request.session_id)
        .ok_or_else(|| "Terminal session not found".to_string())?;

    session
        .master
        .resize(PtySize {
            rows: request.rows.max(1),
            cols: request.cols.max(1),
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn terminal_kill(state: tauri::State<TerminalState>, session_id: String) -> Result<(), String> {
    let mut sessions = state.0.lock().map_err(|error| error.to_string())?;
    let mut session = sessions
        .remove(&session_id)
        .ok_or_else(|| "Terminal session not found".to_string())?;

    let _ = session.child.kill();
    Ok(())
}

fn stream_terminal_output(app: AppHandle, session_id: String, mut reader: Box<dyn Read + Send>) {
    thread::spawn(move || {
        let mut buffer = [0_u8; 8192];

        loop {
            let bytes_read = match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(bytes_read) => bytes_read,
                Err(_) => break,
            };
            let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            let _ = app.emit(
                "terminal://output",
                TerminalOutput {
                    session_id: session_id.clone(),
                    data,
                },
            );
        }

        let _ = app.emit("terminal://exit", TerminalExit { session_id });
    });
}

fn resolve_cwd(cwd: Option<&str>) -> Result<PathBuf, String> {
    let path = match cwd {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().map_err(|error| error.to_string())?,
    };
    let canonical = path.canonicalize().map_err(|error| error.to_string())?;

    if canonical.is_dir() {
        Ok(canonical)
    } else {
        Err("Terminal cwd is not a folder".to_string())
    }
}

fn default_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}
