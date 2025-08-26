// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;

/// Supported stacks for dependency management
#[derive(Clone, Copy)]
enum Stack {
    Node,
    Python,
    Php,
    Ruby,
}

/// Information about a dependency
pub struct DependencyInfo {
    pub name: String,
    pub current_version: String,
    pub latest_version: Option<String>,
}

/// Detect the project stack based on marker files
fn detect_stack(dir: &Path) -> Option<Stack> {
    if dir.join("package.json").exists() {
        Some(Stack::Node)
    } else if dir.join("requirements.txt").exists() {
        Some(Stack::Python)
    } else if dir.join("composer.json").exists() {
        Some(Stack::Php)
    } else if dir.join("Gemfile").exists() {
        Some(Stack::Ruby)
    } else {
        None
    }
}

/// List dependencies with their current versions
fn list_current_dependencies(dir: &Path, stack: Stack) -> Vec<DependencyInfo> {
    match stack {
        Stack::Node => list_node_dependencies(dir),
        Stack::Python => list_python_dependencies(dir),
        Stack::Php => list_php_dependencies(dir),
        Stack::Ruby => list_ruby_dependencies(dir),
    }
}

fn list_node_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = dir.join("package.json");
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut deps = Vec::new();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(obj) = json.get("dependencies").and_then(|v| v.as_object()) {
            for (name, ver) in obj {
                deps.push(DependencyInfo {
                    name: name.clone(),
                    current_version: ver.as_str().unwrap_or_default().to_string(),
                    latest_version: None,
                });
            }
        }
        if let Some(obj) = json.get("devDependencies").and_then(|v| v.as_object()) {
            for (name, ver) in obj {
                deps.push(DependencyInfo {
                    name: name.clone(),
                    current_version: ver.as_str().unwrap_or_default().to_string(),
                    latest_version: None,
                });
            }
        }
    }
    deps
}

fn list_python_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = dir.join("requirements.txt");
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut deps = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((name, ver)) = line.split_once("==") {
            deps.push(DependencyInfo {
                name: name.trim().to_string(),
                current_version: ver.trim().to_string(),
                latest_version: None,
            });
        } else {
            deps.push(DependencyInfo {
                name: line.to_string(),
                current_version: String::new(),
                latest_version: None,
            });
        }
    }
    deps
}

fn list_php_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = dir.join("composer.json");
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut deps = Vec::new();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(obj) = json.get("require").and_then(|v| v.as_object()) {
            for (name, ver) in obj {
                deps.push(DependencyInfo {
                    name: name.clone(),
                    current_version: ver.as_str().unwrap_or_default().to_string(),
                    latest_version: None,
                });
            }
        }
        if let Some(obj) = json.get("require-dev").and_then(|v| v.as_object()) {
            for (name, ver) in obj {
                deps.push(DependencyInfo {
                    name: name.clone(),
                    current_version: ver.as_str().unwrap_or_default().to_string(),
                    latest_version: None,
                });
            }
        }
    }
    deps
}

fn list_ruby_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = dir.join("Gemfile");
    let content = fs::read_to_string(path).unwrap_or_default();
    let gem_re = Regex::new(r"^\s*gem\s+['\"]([^'\"]+)['\"](?:,\s*['\"]([^'\"]+)['\"])?").unwrap();
    let mut deps = Vec::new();
    for line in content.lines() {
        if let Some(caps) = gem_re.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let version = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
            deps.push(DependencyInfo { name, current_version: version, latest_version: None });
        }
    }
    deps
}

fn fetch_latest_version(stack: Stack, name: &str) -> Option<String> {
    match stack {
        Stack::Node => {
            Command::new("npm")
                .args(["view", name, "version"])
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .filter(|s| !s.is_empty())
        }
        Stack::Python => {
            let output = Command::new("pip")
                .args(["index", "versions", name])
                .output()
                .ok()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some(rest) = line.strip_prefix("Available versions: ") {
                    return rest.split(',').next().map(|v| v.trim().to_string());
                }
            }
            None
        }
        Stack::Php => {
            Command::new("composer")
                .args(["show", name, "--format=json"])
                .output()
                .ok()
                .and_then(|o| {
                    serde_json::from_slice::<serde_json::Value>(&o.stdout).ok().and_then(|j| {
                        j.get("latest")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .or_else(|| {
                                j.get("versions")
                                    .and_then(|v| v.as_array())
                                    .and_then(|a| a.first())
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string())
                            })
                    })
                })
        }
        Stack::Ruby => {
            Command::new("gem")
                .args(["search", name, "--remote", "--no-verbose"])
                .output()
                .ok()
                .and_then(|o| {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    for line in stdout.lines() {
                        if let Some(rest) = line.strip_prefix(&format!("{} (", name)) {
                            return rest
                                .split(',')
                                .next()
                                .map(|v| v.trim().trim_end_matches(')').to_string());
                        }
                    }
                    None
                })
        }
    }
}

fn update_requirements(dir: &Path, name: &str, version: &str) {
    let path = dir.join("requirements.txt");
    let content = fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mut found = false;
    for line in lines.iter_mut() {
        if line.trim_start().starts_with(&format!("{}==", name)) || line.trim() == name {
            *line = format!("{}=={}", name, version);
            found = true;
        }
    }
    if !found {
        lines.push(format!("{}=={}", name, version));
    }
    let new_content = lines.join("\n");
    let _ = fs::write(path, new_content);
}

fn remove_from_requirements(dir: &Path, name: &str) {
    let path = dir.join("requirements.txt");
    let content = fs::read_to_string(&path).unwrap_or_default();
    let lines: Vec<String> = content
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.starts_with('#') && !t.starts_with(&format!("{}==", name)) && t != name
        })
        .map(|l| l.to_string())
        .collect();
    let _ = fs::write(path, lines.join("\n"));
}

fn update_composer_json(dir: &Path, name: &str, version: &str) {
    let path = dir.join("composer.json");
    let content = fs::read_to_string(&path).unwrap_or_default();
    if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(map) = json.get_mut("require").and_then(|v| v.as_object_mut()) {
            map.insert(name.to_string(), serde_json::Value::String(version.to_string()));
        }
        let _ = fs::write(path, serde_json::to_string_pretty(&json).unwrap_or(content));
    }
}

fn remove_from_composer_json(dir: &Path, name: &str) {
    let path = dir.join("composer.json");
    let content = fs::read_to_string(&path).unwrap_or_default();
    if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
        if let Some(map) = json.get_mut("require").and_then(|v| v.as_object_mut()) {
            map.remove(name);
        }
        if let Some(map) = json.get_mut("require-dev").and_then(|v| v.as_object_mut()) {
            map.remove(name);
        }
        let _ = fs::write(path, serde_json::to_string_pretty(&json).unwrap_or(content));
    }
}

fn update_gemfile(dir: &Path, name: &str, version: &str) {
    let path = dir.join("Gemfile");
    let content = fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let re = Regex::new(&format!(r"^\s*gem\s+['\"]{}['\"]", regex::escape(name))).unwrap();
    let mut found = false;
    let new_line = format!("gem '{}', '{}'", name, version);
    for line in lines.iter_mut() {
        if re.is_match(line) {
            *line = new_line.clone();
            found = true;
        }
    }
    if !found {
        lines.push(new_line);
    }
    let _ = fs::write(path, lines.join("\n"));
}

fn remove_from_gemfile(dir: &Path, name: &str) {
    let path = dir.join("Gemfile");
    let content = fs::read_to_string(&path).unwrap_or_default();
    let re = Regex::new(&format!(r"^\s*gem\s+['\"]{}['\"]", regex::escape(name))).unwrap();
    let lines: Vec<String> = content
        .lines()
        .filter(|l| !re.is_match(l))
        .map(|l| l.to_string())
        .collect();
    let _ = fs::write(path, lines.join("\n"));
}

/// List dependencies showing latest versions
pub fn list(dir: Option<PathBuf>) {
    let dir = dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    if let Some(stack) = detect_stack(&dir) {
        let mut deps = list_current_dependencies(&dir, stack);
        for dep in deps.iter_mut() {
            dep.latest_version = fetch_latest_version(stack, &dep.name);
        }
        if deps.is_empty() {
            println!("Nenhuma dependência encontrada.");
            return;
        }
        println!("Dependência | Versão Atual | Última Versão");
        println!("-----------|---------------|--------------");
        for d in deps {
            let latest = d.latest_version.unwrap_or_else(|| "?".into());
            println!("{} | {} | {}", d.name, d.current_version, latest);
        }
    } else {
        eprintln!("Stack não suportada ou arquivos de manifesto não encontrados.");
    }
}

/// Add dependency using package manager and update manifest
pub fn add(dir: Option<PathBuf>, name: &str) {
    let dir = dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    match detect_stack(&dir) {
        Some(Stack::Node) => {
            let status = Command::new("npm")
                .arg("install")
                .arg(format!("{}@latest", name))
                .current_dir(&dir)
                .status();
            if let Ok(st) = status { if st.success() { println!("Dependência adicionada."); } }
        }
        Some(Stack::Python) => {
            let status = Command::new("pip")
                .arg("install")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status { if st.success() {
                if let Some(v) = fetch_latest_version(Stack::Python, name) {
                    update_requirements(&dir, name, &v);
                }
                println!("Dependência adicionada.");
            }}
        }
        Some(Stack::Php) => {
            let status = Command::new("composer")
                .arg("require")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status {
                if st.success() {
                    if let Some(v) = fetch_latest_version(Stack::Php, name) {
                        update_composer_json(&dir, name, &v);
                    }
                    println!("Dependência adicionada.");
                }
            }
        }
        Some(Stack::Ruby) => {
            let status = Command::new("bundle")
                .arg("add")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status {
                if st.success() {
                    if let Some(v) = fetch_latest_version(Stack::Ruby, name) {
                        update_gemfile(&dir, name, &v);
                    }
                    println!("Dependência adicionada.");
                }
            }
        }
        None => eprintln!("Stack não suportada."),
    }
}

/// Update dependency to latest version
pub fn update(dir: Option<PathBuf>, name: &str) {
    let dir = dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    match detect_stack(&dir) {
        Some(Stack::Node) => {
            let status = Command::new("npm")
                .arg("install")
                .arg(format!("{}@latest", name))
                .current_dir(&dir)
                .status();
            if let Ok(st) = status { if st.success() { println!("Dependência atualizada."); } }
        }
        Some(Stack::Python) => {
            let status = Command::new("pip")
                .arg("install")
                .arg("--upgrade")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status { if st.success() {
                if let Some(v) = fetch_latest_version(Stack::Python, name) {
                    update_requirements(&dir, name, &v);
                }
                println!("Dependência atualizada.");
            }}
        }
        Some(Stack::Php) => {
            let status = Command::new("composer")
                .arg("update")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status {
                if st.success() {
                    if let Some(v) = fetch_latest_version(Stack::Php, name) {
                        update_composer_json(&dir, name, &v);
                    }
                    println!("Dependência atualizada.");
                }
            }
        }
        Some(Stack::Ruby) => {
            let status = Command::new("bundle")
                .arg("update")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status {
                if st.success() {
                    if let Some(v) = fetch_latest_version(Stack::Ruby, name) {
                        update_gemfile(&dir, name, &v);
                    }
                    println!("Dependência atualizada.");
                }
            }
        }
        None => eprintln!("Stack não suportada."),
    }
}

/// Remove dependency from project
pub fn remove(dir: Option<PathBuf>, name: &str) {
    let dir = dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    match detect_stack(&dir) {
        Some(Stack::Node) => {
            let status = Command::new("npm")
                .arg("uninstall")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status { if st.success() { println!("Dependência removida."); } }
        }
        Some(Stack::Python) => {
            let status = Command::new("pip")
                .arg("uninstall")
                .arg("-y")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status { if st.success() {
                remove_from_requirements(&dir, name);
                println!("Dependência removida.");
            }}
        }
        Some(Stack::Php) => {
            let status = Command::new("composer")
                .arg("remove")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status {
                if st.success() {
                    remove_from_composer_json(&dir, name);
                    println!("Dependência removida.");
                }
            }
        }
        Some(Stack::Ruby) => {
            let status = Command::new("bundle")
                .arg("remove")
                .arg(name)
                .current_dir(&dir)
                .status();
            if let Ok(st) = status {
                if st.success() {
                    remove_from_gemfile(&dir, name);
                    println!("Dependência removida.");
                }
            }
        }
        None => eprintln!("Stack não suportada."),
    }
}

/// Helper for analyzer: return dependencies with latest versions
pub fn gather_with_latest(dir: &Path) -> Vec<DependencyInfo> {
    if let Some(stack) = detect_stack(dir) {
        let mut deps = list_current_dependencies(dir, stack);
        for dep in deps.iter_mut() {
            dep.latest_version = fetch_latest_version(stack, &dep.name);
        }
        deps
    } else {
        Vec::new()
    }
}
