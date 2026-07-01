use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::{
    fs,
    path::{Component, Path, PathBuf},
    sync::Mutex,
};
use tauri::{AppHandle, Emitter, Manager};

const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;
const IGNORED_DIRS: &[&str] = &[".git", "node_modules", "vendor", "target", "dist", "build"];

pub struct WorkspaceWatcher(pub Mutex<Option<RecommendedWatcher>>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTreeEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<FileTreeEntry>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadFileResponse {
    pub path: String,
    pub content: Option<String>,
    pub size: u64,
    pub too_large: bool,
    pub binary: bool,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFsEvent {
    pub kind: String,
    pub paths: Vec<String>,
}

#[tauri::command]
pub fn list_workspace_tree(root_path: String) -> Result<Vec<FileTreeEntry>, String> {
    let root = canonical_dir(&root_path)?;
    read_dir_entries(&root, &root)
}

#[tauri::command]
pub fn read_workspace_file(root_path: String, path: String) -> Result<ReadFileResponse, String> {
    let root = canonical_dir(&root_path)?;
    let file_path = canonical_child(&root, &path)?;
    let metadata = fs::metadata(&file_path).map_err(|error| error.to_string())?;

    if !metadata.is_file() {
        return Err("Path is not a file".to_string());
    }

    let size = metadata.len();
    if size > MAX_FILE_BYTES {
        return Ok(ReadFileResponse {
            path,
            content: None,
            size,
            too_large: true,
            binary: false,
        });
    }

    let bytes = fs::read(&file_path).map_err(|error| error.to_string())?;
    if bytes.contains(&0) {
        return Ok(ReadFileResponse {
            path,
            content: None,
            size,
            too_large: false,
            binary: true,
        });
    }

    let content = String::from_utf8(bytes).map_err(|_| "File is not valid UTF-8".to_string())?;

    Ok(ReadFileResponse {
        path,
        content: Some(content),
        size,
        too_large: false,
        binary: false,
    })
}

#[tauri::command]
pub fn write_workspace_file(
    root_path: String,
    path: String,
    content: String,
) -> Result<(), String> {
    let root = canonical_dir(&root_path)?;
    let file_path = canonical_child(&root, &path)?;

    if !file_path.is_file() {
        return Err("Path is not a file".to_string());
    }

    fs::write(file_path, content).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_workspace_file(root_path: String, path: String) -> Result<(), String> {
    let root = canonical_dir(&root_path)?;
    let file_path = child_for_create(&root, &path)?;

    if file_path.exists() {
        return Err("File already exists".to_string());
    }

    fs::File::create(file_path)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_workspace_folder(root_path: String, path: String) -> Result<(), String> {
    let root = canonical_dir(&root_path)?;
    let folder_path = child_for_create(&root, &path)?;

    if folder_path.exists() {
        return Err("Folder already exists".to_string());
    }

    fs::create_dir(folder_path).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn rename_workspace_entry(
    root_path: String,
    path: String,
    new_name: String,
) -> Result<(), String> {
    let root = canonical_dir(&root_path)?;
    validate_file_name(&new_name)?;
    let source = canonical_child(&root, &path)?;

    if source == root {
        return Err("Cannot rename workspace root".to_string());
    }

    let target = source
        .parent()
        .ok_or_else(|| "Cannot rename workspace root".to_string())?
        .join(new_name);

    if target.exists() {
        return Err("Target already exists".to_string());
    }

    fs::rename(source, target).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_workspace_entry(root_path: String, path: String) -> Result<(), String> {
    let root = canonical_dir(&root_path)?;
    let entry_path = canonical_child(&root, &path)?;

    if entry_path == root {
        return Err("Cannot delete workspace root".to_string());
    }

    let metadata = fs::metadata(&entry_path).map_err(|error| error.to_string())?;
    if metadata.is_dir() {
        fs::remove_dir_all(entry_path).map_err(|error| error.to_string())
    } else {
        fs::remove_file(entry_path).map_err(|error| error.to_string())
    }
}

#[tauri::command]
pub fn start_workspace_watch(app: AppHandle, root_path: String) -> Result<(), String> {
    let root = canonical_dir(&root_path)?;
    let emit_app = app.clone();

    let mut watcher = RecommendedWatcher::new(
        move |result: notify::Result<notify::Event>| {
            if let Ok(event) = result {
                let paths = event
                    .paths
                    .into_iter()
                    .filter(|path| !has_ignored_component(path))
                    .map(|path| path.to_string_lossy().to_string())
                    .collect::<Vec<_>>();

                if paths.is_empty() {
                    return;
                }

                let payload = WorkspaceFsEvent {
                    kind: event_kind_label(&event.kind).to_string(),
                    paths,
                };
                let _ = emit_app.emit("workspace://changed", payload);
            }
        },
        Config::default(),
    )
    .map_err(|error| error.to_string())?;

    watcher
        .watch(&root, RecursiveMode::Recursive)
        .map_err(|error| error.to_string())?;

    let state = app.state::<WorkspaceWatcher>();
    let mut active_watcher = state.0.lock().map_err(|error| error.to_string())?;
    *active_watcher = Some(watcher);
    Ok(())
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
    ensure_inside(root, &canonical)?;
    Ok(canonical)
}

fn child_for_create(root: &Path, path: &str) -> Result<PathBuf, String> {
    let candidate = PathBuf::from(path);
    let parent = candidate
        .parent()
        .ok_or_else(|| "Path must include a parent folder".to_string())?;
    let canonical_parent = fs::canonicalize(parent).map_err(|error| error.to_string())?;
    ensure_inside(root, &canonical_parent)?;

    let file_name = candidate
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;
    validate_file_name(file_name)?;

    Ok(canonical_parent.join(file_name))
}

fn ensure_inside(root: &Path, path: &Path) -> Result<(), String> {
    if path.starts_with(root) {
        Ok(())
    } else {
        Err("Path is outside the active workspace".to_string())
    }
}

fn validate_file_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() || name.contains('/') || name.contains('\\') {
        return Err("Invalid file name".to_string());
    }

    Ok(())
}

fn read_dir_entries(root: &Path, dir: &Path) -> Result<Vec<FileTreeEntry>, String> {
    let mut entries = fs::read_dir(dir)
        .map_err(|error| error.to_string())?
        .filter_map(Result::ok)
        .filter(|entry| !is_ignored_dir(entry))
        .map(|entry| {
            let path = entry.path();
            let metadata = entry.metadata().map_err(|error| error.to_string())?;
            let is_dir = metadata.is_dir();
            let children = if is_dir {
                Some(read_dir_entries(root, &path)?)
            } else {
                None
            };

            Ok(FileTreeEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: path.to_string_lossy().to_string(),
                is_dir,
                children,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    entries.sort_by(|left, right| {
        right
            .is_dir
            .cmp(&left.is_dir)
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
    });

    for entry in &entries {
        ensure_inside(root, Path::new(&entry.path))?;
    }

    Ok(entries)
}

fn is_ignored_dir(entry: &fs::DirEntry) -> bool {
    entry
        .metadata()
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
        && IGNORED_DIRS.contains(&entry.file_name().to_string_lossy().as_ref())
}

fn has_ignored_component(path: &Path) -> bool {
    path.components().any(|component| match component {
        Component::Normal(name) => IGNORED_DIRS.contains(&name.to_string_lossy().as_ref()),
        _ => false,
    })
}

fn event_kind_label(kind: &EventKind) -> &'static str {
    match kind {
        EventKind::Create(_) => "created",
        EventKind::Modify(_) => "modified",
        EventKind::Remove(_) => "removed",
        _ => "changed",
    }
}
