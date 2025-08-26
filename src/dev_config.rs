// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stack {
    Rust,
    Node,
    Python,
    Go,
    JavaMaven,
    JavaGradle,
    Unknown,
}

impl Stack {
    fn detect(dir: &Path) -> Self {
        if dir.join("Cargo.toml").exists() {
            Stack::Rust
        } else if dir.join("package.json").exists() {
            Stack::Node
        } else if dir.join("pyproject.toml").exists() || dir.join("requirements.txt").exists() {
            Stack::Python
        } else if dir.join("go.mod").exists() {
            Stack::Go
        } else if dir.join("pom.xml").exists() {
            Stack::JavaMaven
        } else if dir.join("build.gradle").exists() || dir.join("build.gradle.kts").exists() {
            Stack::JavaGradle
        } else {
            Stack::Unknown
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Stack::Rust => "Rust",
            Stack::Node => "Node.js",
            Stack::Python => "Python",
            Stack::Go => "Go",
            Stack::JavaMaven => "Java (Maven)",
            Stack::JavaGradle => "Java (Gradle)",
            Stack::Unknown => "Desconhecida",
        };
        write!(f, "{name}")
    }
}

#[derive(Serialize, Deserialize, Default)]
struct Config(BTreeMap<String, String>);

impl Config {
    fn load(path: &Path) -> Self {
        if let Ok(data) = fs::read_to_string(path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, data)
    }
}

fn config_path(project_dir: &Path) -> PathBuf {
    project_dir.join(".dx").join("config.json")
}

fn project_dir(dir: Option<PathBuf>) -> PathBuf {
    dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

pub fn list(dir: Option<PathBuf>) {
    let project_dir = project_dir(dir);
    let stack = Stack::detect(&project_dir);
    println!("Stack detectada: {}", stack);

    let path = config_path(&project_dir);
    let cfg = Config::load(&path);
    if cfg.0.is_empty() {
        println!("Nenhuma configuração encontrada.");
    } else {
        for (k, v) in cfg.0 {
            println!("- {k} = {v}");
        }
    }
}

pub fn add(dir: Option<PathBuf>, key: String, value: String) {
    let project_dir = project_dir(dir);
    let stack = Stack::detect(&project_dir);
    println!("Stack detectada: {}", stack);

    let path = config_path(&project_dir);
    let mut cfg = Config::load(&path);
    if cfg.0.contains_key(&key) {
        println!("Configuração '{key}' já existe.");
        return;
    }
    cfg.0.insert(key.clone(), value);
    if let Err(e) = cfg.save(&path) {
        eprintln!("Erro ao salvar configurações: {e}");
    } else {
        println!("Configuração '{key}' criada.");
    }
}

pub fn update(dir: Option<PathBuf>, key: String, value: String) {
    let project_dir = project_dir(dir);
    let stack = Stack::detect(&project_dir);
    println!("Stack detectada: {}", stack);

    let path = config_path(&project_dir);
    let mut cfg = Config::load(&path);
    if !cfg.0.contains_key(&key) {
        println!("Configuração '{key}' não existe.");
        return;
    }
    cfg.0.insert(key.clone(), value);
    if let Err(e) = cfg.save(&path) {
        eprintln!("Erro ao salvar configurações: {e}");
    } else {
        println!("Configuração '{key}' atualizada.");
    }
}

pub fn delete(dir: Option<PathBuf>, key: String) {
    let project_dir = project_dir(dir);
    let stack = Stack::detect(&project_dir);
    println!("Stack detectada: {}", stack);

    let path = config_path(&project_dir);
    let mut cfg = Config::load(&path);
    if cfg.0.remove(&key).is_some() {
        if let Err(e) = cfg.save(&path) {
            eprintln!("Erro ao salvar configurações: {e}");
        } else {
            println!("Configuração '{key}' removida.");
        }
    } else {
        println!("Configuração '{key}' não existe.");
    }
}
