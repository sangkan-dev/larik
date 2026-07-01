use serde::Serialize;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDetectionResponse {
    pub root_path: String,
    pub detected: Vec<DetectedProject>,
    pub actions: Vec<ProjectAction>,
    pub env: EnvStatus,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedProject {
    pub kind: String,
    pub name: String,
    pub confidence: f32,
    pub detected_files: Vec<String>,
    pub details: Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAction {
    pub id: String,
    pub label: String,
    pub command: String,
    pub cwd: String,
    pub category: String,
    pub destructive: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvStatus {
    pub has_env: bool,
    pub has_env_example: bool,
    pub missing_keys: Vec<String>,
    pub empty_keys: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

impl PackageManager {
    fn label(self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
            PackageManager::Bun => "bun",
        }
    }

    fn install_command(self) -> String {
        format!("{} install", self.label())
    }

    fn run_command(self, script: &str) -> String {
        match self {
            PackageManager::Yarn => format!("yarn {script}"),
            PackageManager::Bun => format!("bun run {script}"),
            PackageManager::Npm | PackageManager::Pnpm => {
                format!("{} run {script}", self.label())
            }
        }
    }
}

#[tauri::command]
pub fn detect_project(root_path: String) -> Result<ProjectDetectionResponse, String> {
    let root = canonical_dir(&root_path)?;
    let cwd = root.to_string_lossy().to_string();
    let mut detected = Vec::new();
    let mut actions = Vec::new();
    let mut warnings = Vec::new();
    let env = detect_env(&root, &mut warnings);

    detect_node(&root, &cwd, &mut detected, &mut actions, &mut warnings);
    detect_php_laravel(&root, &cwd, &mut detected, &mut actions, &mut warnings);
    detect_rust(&root, &cwd, &mut detected, &mut actions, &mut warnings);
    detect_go(&root, &cwd, &mut detected, &mut actions, &mut warnings);
    detect_docker_compose(&root, &cwd, &mut detected, &mut actions);

    Ok(ProjectDetectionResponse {
        root_path: cwd,
        detected,
        actions,
        env,
        warnings,
    })
}

fn detect_node(
    root: &Path,
    cwd: &str,
    detected: &mut Vec<DetectedProject>,
    actions: &mut Vec<ProjectAction>,
    warnings: &mut Vec<String>,
) {
    let package_path = root.join("package.json");
    if !package_path.is_file() {
        return;
    }

    let mut detected_files = vec!["package.json".to_string()];
    let package_manager = detect_package_manager(root, &mut detected_files);
    let package_json = match read_json_file(&package_path) {
        Ok(value) => value,
        Err(error) => {
            warnings.push(format!("Cannot parse package.json: {error}"));
            Value::Null
        }
    };
    let name = package_json
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("Node project")
        .to_string();
    let scripts = package_json
        .get("scripts")
        .and_then(Value::as_object)
        .map(|items| {
            items
                .keys()
                .map(ToString::to_string)
                .collect::<BTreeSet<String>>()
        })
        .unwrap_or_default();
    let framework = detect_node_framework(root, &package_json, &mut detected_files);

    actions.push(ProjectAction {
        id: "node.install".to_string(),
        label: "Install dependencies".to_string(),
        command: package_manager.install_command(),
        cwd: cwd.to_string(),
        category: "Node".to_string(),
        destructive: false,
    });

    for script in ["dev", "build", "test"] {
        if scripts.contains(script) {
            actions.push(ProjectAction {
                id: format!("node.script.{script}"),
                label: format!("Run {script}"),
                command: package_manager.run_command(script),
                cwd: cwd.to_string(),
                category: "Node".to_string(),
                destructive: false,
            });
        }
    }

    let mut details = BTreeMap::new();
    details.insert("framework", json!(framework));
    details.insert("packageManager", json!(package_manager.label()));
    details.insert(
        "scripts",
        json!(scripts.into_iter().collect::<Vec<String>>()),
    );

    detected.push(DetectedProject {
        kind: "node".to_string(),
        name,
        confidence: if framework == "Generic Node" {
            0.7
        } else {
            0.9
        },
        detected_files,
        details: json!(details),
    });
}

fn detect_php_laravel(
    root: &Path,
    cwd: &str,
    detected: &mut Vec<DetectedProject>,
    actions: &mut Vec<ProjectAction>,
    warnings: &mut Vec<String>,
) {
    let composer_path = root.join("composer.json");
    if !composer_path.is_file() {
        return;
    }

    let mut detected_files = vec!["composer.json".to_string()];
    let composer_json = match read_json_file(&composer_path) {
        Ok(value) => value,
        Err(error) => {
            warnings.push(format!("Cannot parse composer.json: {error}"));
            Value::Null
        }
    };
    let name = composer_json
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("PHP project")
        .to_string();
    let has_artisan = push_if_exists(root, "artisan", &mut detected_files);
    let has_app = push_if_exists(root, "app", &mut detected_files);
    let has_routes = push_if_exists(root, "routes", &mut detected_files);
    let has_migrations = push_if_exists(root, "database/migrations", &mut detected_files);
    let is_laravel = has_artisan || (has_app && has_routes && has_migrations);

    actions.push(ProjectAction {
        id: "php.composer.install".to_string(),
        label: "Composer install".to_string(),
        command: "composer install".to_string(),
        cwd: cwd.to_string(),
        category: "PHP".to_string(),
        destructive: false,
    });

    if is_laravel {
        for (id, label, command) in [
            ("php.laravel.serve", "Artisan serve", "php artisan serve"),
            (
                "php.laravel.migrate",
                "Artisan migrate",
                "php artisan migrate",
            ),
            (
                "php.laravel.queue",
                "Artisan queue worker",
                "php artisan queue:work",
            ),
            ("php.laravel.test", "Artisan test", "php artisan test"),
        ] {
            actions.push(ProjectAction {
                id: id.to_string(),
                label: label.to_string(),
                command: command.to_string(),
                cwd: cwd.to_string(),
                category: "Laravel".to_string(),
                destructive: false,
            });
        }
    }

    detected.push(DetectedProject {
        kind: if is_laravel { "laravel" } else { "php" }.to_string(),
        name,
        confidence: if is_laravel { 0.9 } else { 0.7 },
        detected_files,
        details: json!({
            "hasArtisan": has_artisan,
            "hasApp": has_app,
            "hasRoutes": has_routes,
            "hasMigrations": has_migrations,
        }),
    });
}

fn detect_rust(
    root: &Path,
    cwd: &str,
    detected: &mut Vec<DetectedProject>,
    actions: &mut Vec<ProjectAction>,
    warnings: &mut Vec<String>,
) {
    let cargo_path = root.join("Cargo.toml");
    if !cargo_path.is_file() {
        return;
    }

    let manifest = fs::read_to_string(&cargo_path).unwrap_or_else(|error| {
        warnings.push(format!("Cannot read Cargo.toml: {error}"));
        String::new()
    });
    let package_name = parse_cargo_package_name(&manifest).unwrap_or_else(|| "Rust project".into());

    for (id, label, command) in [
        ("rust.check", "Cargo check", "cargo check"),
        ("rust.run", "Cargo run", "cargo run"),
        ("rust.test", "Cargo test", "cargo test"),
        ("rust.build", "Cargo build", "cargo build"),
    ] {
        actions.push(ProjectAction {
            id: id.to_string(),
            label: label.to_string(),
            command: command.to_string(),
            cwd: cwd.to_string(),
            category: "Rust".to_string(),
            destructive: false,
        });
    }

    detected.push(DetectedProject {
        kind: "rust".to_string(),
        name: package_name,
        confidence: 0.9,
        detected_files: vec!["Cargo.toml".to_string()],
        details: json!({}),
    });
}

fn detect_go(
    root: &Path,
    cwd: &str,
    detected: &mut Vec<DetectedProject>,
    actions: &mut Vec<ProjectAction>,
    warnings: &mut Vec<String>,
) {
    let go_mod_path = root.join("go.mod");
    if !go_mod_path.is_file() {
        return;
    }

    let go_mod = fs::read_to_string(&go_mod_path).unwrap_or_else(|error| {
        warnings.push(format!("Cannot read go.mod: {error}"));
        String::new()
    });
    let module_name = parse_go_module_name(&go_mod).unwrap_or_else(|| "Go project".into());

    for (id, label, command) in [
        ("go.run", "Go run", "go run ."),
        ("go.test", "Go test", "go test ./..."),
        ("go.build", "Go build", "go build ./..."),
    ] {
        actions.push(ProjectAction {
            id: id.to_string(),
            label: label.to_string(),
            command: command.to_string(),
            cwd: cwd.to_string(),
            category: "Go".to_string(),
            destructive: false,
        });
    }

    detected.push(DetectedProject {
        kind: "go".to_string(),
        name: module_name,
        confidence: 0.9,
        detected_files: vec!["go.mod".to_string()],
        details: json!({}),
    });
}

fn detect_docker_compose(
    root: &Path,
    cwd: &str,
    detected: &mut Vec<DetectedProject>,
    actions: &mut Vec<ProjectAction>,
) {
    let mut detected_files = Vec::new();
    push_if_exists(root, "docker-compose.yml", &mut detected_files);
    push_if_exists(root, "compose.yml", &mut detected_files);

    if detected_files.is_empty() {
        return;
    }

    for (id, label, command, destructive) in [
        (
            "docker.compose.up",
            "Compose up",
            "docker compose up",
            false,
        ),
        (
            "docker.compose.upDetached",
            "Compose up detached",
            "docker compose up -d",
            false,
        ),
        (
            "docker.compose.down",
            "Compose down",
            "docker compose down",
            true,
        ),
        (
            "docker.compose.logs",
            "Compose logs",
            "docker compose logs -f",
            false,
        ),
        (
            "docker.compose.ps",
            "Compose ps",
            "docker compose ps",
            false,
        ),
    ] {
        actions.push(ProjectAction {
            id: id.to_string(),
            label: label.to_string(),
            command: command.to_string(),
            cwd: cwd.to_string(),
            category: "Docker".to_string(),
            destructive,
        });
    }

    detected.push(DetectedProject {
        kind: "docker-compose".to_string(),
        name: "Docker Compose".to_string(),
        confidence: 0.85,
        detected_files,
        details: json!({}),
    });
}

fn detect_env(root: &Path, warnings: &mut Vec<String>) -> EnvStatus {
    let env_path = root.join(".env");
    let example_path = root.join(".env.example");
    let has_env = env_path.is_file();
    let has_env_example = example_path.is_file();
    let env_keys = if has_env {
        parse_env_file(&env_path, warnings)
    } else {
        BTreeMap::new()
    };
    let example_keys = if has_env_example {
        parse_env_file(&example_path, warnings)
    } else {
        BTreeMap::new()
    };

    if has_env_example && !has_env {
        warnings.push(".env is missing while .env.example exists".to_string());
    }

    let missing_keys = example_keys
        .keys()
        .filter(|key| !env_keys.contains_key(*key))
        .cloned()
        .collect::<Vec<_>>();
    let empty_keys = env_keys
        .iter()
        .filter_map(|(key, value)| {
            let trimmed = value.trim();
            if trimmed.is_empty() || trimmed == "\"\"" || trimmed == "''" {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    EnvStatus {
        has_env,
        has_env_example,
        missing_keys,
        empty_keys,
    }
}

fn detect_package_manager(root: &Path, detected_files: &mut Vec<String>) -> PackageManager {
    if push_if_exists(root, "pnpm-lock.yaml", detected_files) {
        return PackageManager::Pnpm;
    }
    if push_if_exists(root, "bun.lockb", detected_files)
        || push_if_exists(root, "bun.lock", detected_files)
    {
        return PackageManager::Bun;
    }
    if push_if_exists(root, "yarn.lock", detected_files) {
        return PackageManager::Yarn;
    }
    if push_if_exists(root, "package-lock.json", detected_files) {
        return PackageManager::Npm;
    }

    PackageManager::Npm
}

fn detect_node_framework(
    root: &Path,
    package_json: &Value,
    detected_files: &mut Vec<String>,
) -> String {
    let dependencies = dependency_names(package_json);
    let has_sveltekit = dependencies.contains("@sveltejs/kit");
    let has_vite = dependencies.contains("vite") || root.join("vite.config.js").is_file();
    let has_next = dependencies.contains("next") || root.join("next.config.js").is_file();
    let has_express = dependencies.contains("express");

    push_if_exists(root, "svelte.config.js", detected_files);
    push_if_exists(root, "vite.config.js", detected_files);
    push_if_exists(root, "next.config.js", detected_files);

    if has_sveltekit {
        "SvelteKit".to_string()
    } else if has_next {
        "Next.js".to_string()
    } else if has_vite {
        "Vite".to_string()
    } else if has_express {
        "Express".to_string()
    } else {
        "Generic Node".to_string()
    }
}

fn dependency_names(package_json: &Value) -> BTreeSet<String> {
    ["dependencies", "devDependencies", "peerDependencies"]
        .into_iter()
        .filter_map(|key| package_json.get(key).and_then(Value::as_object))
        .flat_map(|items| items.keys().map(ToString::to_string))
        .collect()
}

fn parse_env_file(path: &Path, warnings: &mut Vec<String>) -> BTreeMap<String, String> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            warnings.push(format!("Cannot read {}: {error}", path.display()));
            return BTreeMap::new();
        }
    };

    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }
            let (key, value) = trimmed.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .filter(|(key, _)| !key.is_empty())
        .collect()
}

fn parse_cargo_package_name(manifest: &str) -> Option<String> {
    let mut in_package = false;

    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_package = trimmed == "[package]";
            continue;
        }
        if in_package {
            if let Some((key, value)) = trimmed.split_once('=') {
                if key.trim() == "name" {
                    return Some(value.trim().trim_matches('"').to_string());
                }
            }
        }
    }

    None
}

fn parse_go_module_name(go_mod: &str) -> Option<String> {
    go_mod.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix("module ")
            .map(|module_name| module_name.trim().to_string())
    })
}

fn read_json_file(path: &Path) -> Result<Value, String> {
    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&content).map_err(|error| error.to_string())
}

fn push_if_exists(root: &Path, relative_path: &str, detected_files: &mut Vec<String>) -> bool {
    if root.join(relative_path).exists() {
        detected_files.push(relative_path.to_string());
        true
    } else {
        false
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
