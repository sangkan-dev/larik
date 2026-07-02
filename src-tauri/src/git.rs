use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitStatusResponse {
    pub root_path: String,
    pub is_repo: bool,
    pub branch: Option<String>,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub changed_files: Vec<GitChangedFile>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiffResponse {
    pub path: String,
    pub staged: bool,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitChangedFile {
    pub path: String,
    pub absolute_path: String,
    pub staged: bool,
    pub unstaged: bool,
    pub untracked: bool,
    pub kind: String,
    pub index_status: String,
    pub worktree_status: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitFileRequest {
    pub root_path: String,
    pub path: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiffRequest {
    pub root_path: String,
    pub path: String,
    pub staged: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitCommitRequest {
    pub root_path: String,
    pub message: String,
}

#[tauri::command]
pub fn git_status(root_path: String) -> Result<GitStatusResponse, String> {
    let root = canonical_dir(&root_path)?;
    let root_label = root.to_string_lossy().to_string();

    if !is_git_repo(&root)? {
        return Ok(GitStatusResponse {
            root_path: root_label,
            is_repo: false,
            branch: None,
            upstream: None,
            ahead: 0,
            behind: 0,
            changed_files: Vec::new(),
        });
    }

    let output = run_git(&root, ["status", "--porcelain=v1", "--branch"])?;
    let mut lines = output.lines();
    let branch_line = lines.next().unwrap_or_default();
    let (branch, upstream, ahead, behind) = parse_branch_line(branch_line);
    let changed_files = lines
        .filter_map(|line| parse_status_line(&root, line))
        .collect();

    Ok(GitStatusResponse {
        root_path: root_label,
        is_repo: true,
        branch,
        upstream,
        ahead,
        behind,
        changed_files,
    })
}

#[tauri::command]
pub fn git_stage_file(request: GitFileRequest) -> Result<GitStatusResponse, String> {
    let root = canonical_git_root(&request.root_path)?;
    validate_git_path(&request.path)?;
    run_git(&root, ["add", "--", request.path.as_str()])?;
    git_status(root.to_string_lossy().to_string())
}

#[tauri::command]
pub fn git_unstage_file(request: GitFileRequest) -> Result<GitStatusResponse, String> {
    let root = canonical_git_root(&request.root_path)?;
    validate_git_path(&request.path)?;
    run_git(&root, ["restore", "--staged", "--", request.path.as_str()])?;
    git_status(root.to_string_lossy().to_string())
}

#[tauri::command]
pub fn git_commit(request: GitCommitRequest) -> Result<GitStatusResponse, String> {
    let root = canonical_git_root(&request.root_path)?;
    let message = request.message.trim();
    if message.is_empty() {
        return Err("Commit message is required".to_string());
    }

    run_git(&root, ["commit", "-m", message])?;
    git_status(root.to_string_lossy().to_string())
}

#[tauri::command]
pub fn git_diff_file(request: GitDiffRequest) -> Result<GitDiffResponse, String> {
    let root = canonical_git_root(&request.root_path)?;
    validate_git_path(&request.path)?;
    let content = if request.staged {
        run_git(
            &root,
            [
                "diff",
                "--cached",
                "--no-ext-diff",
                "--",
                request.path.as_str(),
            ],
        )?
    } else {
        run_git(
            &root,
            ["diff", "--no-ext-diff", "--", request.path.as_str()],
        )?
    };

    Ok(GitDiffResponse {
        path: request.path,
        staged: request.staged,
        content,
    })
}

#[tauri::command]
pub fn git_generate_commit_message(root_path: String) -> Result<String, String> {
    let status = git_status(root_path)?;
    if !status.is_repo {
        return Err("Workspace is not a Git repository".to_string());
    }
    let staged_files = status
        .changed_files
        .iter()
        .filter(|file| file.staged)
        .collect::<Vec<_>>();
    let files = if staged_files.is_empty() {
        status.changed_files.iter().collect::<Vec<_>>()
    } else {
        staged_files
    };

    if files.is_empty() {
        return Err("No changes to summarize".to_string());
    }

    let prefix = if files.iter().any(|file| file.kind == "added") {
        "Add"
    } else if files.iter().all(|file| file.kind == "deleted") {
        "Remove"
    } else {
        "Update"
    };
    let scope = summarize_paths(files.iter().map(|file| file.path.as_str()).collect());

    Ok(format!("{prefix} {scope}"))
}

fn canonical_git_root(root_path: &str) -> Result<PathBuf, String> {
    let root = canonical_dir(root_path)?;
    if is_git_repo(&root)? {
        Ok(root)
    } else {
        Err("Workspace is not a Git repository".to_string())
    }
}

fn is_git_repo(root: &Path) -> Result<bool, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map_err(|error| error.to_string())?;

    Ok(output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true")
}

fn validate_git_path(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if path.is_absolute() {
        return Err("Git path must be relative".to_string());
    }
    if path
        .components()
        .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return Err("Git path is invalid".to_string());
    }

    Ok(())
}

fn run_git<const N: usize>(root: &Path, args: [&str; N]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "Git command failed".to_string()
        } else {
            stderr
        });
    }

    String::from_utf8(output.stdout).map_err(|_| "Git output is not valid UTF-8".to_string())
}

fn parse_branch_line(line: &str) -> (Option<String>, Option<String>, u32, u32) {
    let trimmed = line.strip_prefix("## ").unwrap_or(line);
    let (branch_part, tracking_part) = trimmed
        .split_once(" [")
        .map(|(branch, tracking)| (branch, tracking.trim_end_matches(']')))
        .unwrap_or((trimmed, ""));
    let (branch, upstream) = branch_part
        .split_once("...")
        .map(|(branch, upstream)| (branch.to_string(), Some(upstream.to_string())))
        .unwrap_or((branch_part.to_string(), None));
    let ahead = parse_tracking_count(tracking_part, "ahead");
    let behind = parse_tracking_count(tracking_part, "behind");

    (Some(branch), upstream, ahead, behind)
}

fn parse_tracking_count(tracking: &str, label: &str) -> u32 {
    tracking
        .split(',')
        .find_map(|part| {
            let trimmed = part.trim();
            trimmed
                .strip_prefix(label)
                .and_then(|value| value.trim().parse::<u32>().ok())
        })
        .unwrap_or(0)
}

fn parse_status_line(root: &Path, line: &str) -> Option<GitChangedFile> {
    if line.len() < 4 {
        return None;
    }

    let index_status = line.chars().next()?.to_string();
    let worktree_status = line.chars().nth(1)?.to_string();
    let raw_path = line.get(3..)?.trim();
    let path = raw_path
        .split(" -> ")
        .last()
        .unwrap_or(raw_path)
        .trim_matches('"')
        .to_string();
    let staged = index_status != " " && index_status != "?";
    let unstaged = worktree_status != " " && worktree_status != "?";
    let untracked = index_status == "?" && worktree_status == "?";
    let kind = status_kind(&index_status, &worktree_status).to_string();
    let absolute_path = root.join(&path).to_string_lossy().to_string();

    Some(GitChangedFile {
        path,
        absolute_path,
        staged,
        unstaged,
        untracked,
        kind,
        index_status,
        worktree_status,
    })
}

fn status_kind(index_status: &str, worktree_status: &str) -> &'static str {
    if index_status == "?" && worktree_status == "?" {
        return "untracked";
    }

    for status in [index_status, worktree_status] {
        match status {
            "A" => return "added",
            "M" => return "modified",
            "D" => return "deleted",
            "R" => return "renamed",
            "C" => return "copied",
            "U" => return "conflicted",
            _ => {}
        }
    }

    "changed"
}

fn summarize_paths(paths: Vec<&str>) -> String {
    if paths.len() == 1 {
        return paths[0].to_string();
    }

    let mut top_level = paths
        .iter()
        .filter_map(|path| path.split('/').next())
        .collect::<Vec<_>>();
    top_level.sort_unstable();
    top_level.dedup();

    if top_level.len() == 1 {
        format!("{} files", top_level[0])
    } else {
        format!("{} files", paths.len())
    }
}

fn canonical_dir(path: &str) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path).map_err(|error| error.to_string())?;
    if canonical.is_dir() {
        Ok(canonical)
    } else {
        Err("Workspace path is not a folder".to_string())
    }
}
