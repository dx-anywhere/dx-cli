// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "properties.yaml";

#[derive(ValueEnum, Clone)]
pub enum DevConfigSection {
    Config,
    Env,
    Priority,
}

#[derive(Serialize, Deserialize, Default)]
pub struct StackConfig {
    #[serde(default)]
    pub configs: BTreeMap<String, String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default)]
    pub priorities: BTreeMap<String, i64>,
}

fn config_path(project_dir: &Path, stack: &str) -> PathBuf {
    project_dir.join(".dx").join(stack).join(CONFIG_FILE)
}

fn load_config(project_dir: &Path, stack: &str) -> std::io::Result<StackConfig> {
    let path = config_path(project_dir, stack);
    if path.exists() {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    } else {
        Ok(StackConfig::default())
    }
}

fn save_config(project_dir: &Path, stack: &str, cfg: &StackConfig) -> std::io::Result<()> {
    let path = config_path(project_dir, stack);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_yaml::to_string(cfg)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(path, content)
}

fn resolve_dir(dir: Option<PathBuf>) -> PathBuf {
    dir.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn resolve_stack(stack: Option<String>) -> String {
    stack.unwrap_or_else(|| "default".to_string())
}

pub fn list(dir: Option<PathBuf>, stack: Option<String>) {
    let project_dir = resolve_dir(dir);
    let stack_name = resolve_stack(stack);
    match load_config(&project_dir, &stack_name) {
        Ok(cfg) => match serde_yaml::to_string(&cfg) {
            Ok(yaml) => println!("{}", yaml),
            Err(e) => eprintln!("Erro ao serializar configuração: {}", e),
        },
        Err(e) => eprintln!("Erro ao carregar configuração: {}", e),
    }
}

pub fn set(
    dir: Option<PathBuf>,
    stack: Option<String>,
    section: DevConfigSection,
    key: String,
    value: String,
) {
    let project_dir = resolve_dir(dir);
    let stack_name = resolve_stack(stack);
    let mut cfg = match load_config(&project_dir, &stack_name) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erro ao carregar configuração: {}", e);
            return;
        }
    };

    match section {
        DevConfigSection::Config => {
            cfg.configs.insert(key, value);
        }
        DevConfigSection::Env => {
            cfg.env.insert(key, value);
        }
        DevConfigSection::Priority => match value.parse::<i64>() {
            Ok(v) => {
                cfg.priorities.insert(key, v);
            }
            Err(_) => {
                eprintln!("Valor de prioridade deve ser um número inteiro");
                return;
            }
        },
    }

    if let Err(e) = save_config(&project_dir, &stack_name, &cfg) {
        eprintln!("Erro ao salvar configuração: {}", e);
    }
}

pub fn remove(dir: Option<PathBuf>, stack: Option<String>, section: DevConfigSection, key: String) {
    let project_dir = resolve_dir(dir);
    let stack_name = resolve_stack(stack);
    let mut cfg = match load_config(&project_dir, &stack_name) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erro ao carregar configuração: {}", e);
            return;
        }
    };

    match section {
        DevConfigSection::Config => {
            cfg.configs.remove(&key);
        }
        DevConfigSection::Env => {
            cfg.env.remove(&key);
        }
        DevConfigSection::Priority => {
            cfg.priorities.remove(&key);
        }
    }

    if let Err(e) = save_config(&project_dir, &stack_name, &cfg) {
        eprintln!("Erro ao salvar configuração: {}", e);
    }
}
