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

fn is_git_repo(root: &Path) -> Result<bool, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map_err(|error| error.to_string())?;

    Ok(output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true")
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

fn canonical_dir(path: &str) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path).map_err(|error| error.to_string())?;
    if canonical.is_dir() {
        Ok(canonical)
    } else {
        Err("Workspace path is not a folder".to_string())
    }
}
