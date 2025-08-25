// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors
use std::fs;
use std::path::{Path, PathBuf};

use crate::dev_services;

const START_MARKER: &str = "<!-- dx-cli:badges:start -->";
const END_MARKER: &str = "<!-- dx-cli:badges:end -->";

/// Generate a Markdown line with badges for the given services
pub fn generate_badges_markdown(services: &[String]) -> String {
    use std::collections::HashSet;

    // Build the same badges as the Analyzer report
    let mut badges: HashSet<&str> = HashSet::new();

    for s in services {
        let kl = s.to_lowercase();
        match kl.as_str() {
            // Databases
            "postgres" | "postgresql" => {
                badges.insert("[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Dev_Service-blue?logo=postgresql)](#)");
            }
            "mysql" | "mariadb" => {
                badges.insert(
                    "[![MySQL](https://img.shields.io/badge/MySQL-Dev_Service-blue?logo=mysql)](#)",
                );
            }
            "mongodb" => {
                badges.insert("[![MongoDB](https://img.shields.io/badge/MongoDB-Dev_Service-green?logo=mongodb)](#)");
            }
            // Cache
            "redis" => {
                badges.insert(
                    "[![Redis](https://img.shields.io/badge/Redis-Dev_Service-red?logo=redis)](#)",
                );
            }
            // Streaming (Kafka API)
            "kafka" => {
                badges.insert("[![Kafka](https://img.shields.io/badge/Kafka-Dev_Service-black?logo=apachekafka)](#)");
            }
            // Data processing (Flink detected by jobmanager/taskmanager too)
            "flink" | "jobmanager" | "taskmanager" => {
                badges.insert("[![Apache Flink](https://img.shields.io/badge/Flink-Dev_Service-orange?logo=apacheflink)](#)");
            }
            // Skip tools like kafka-ui
            "kafka-ui" => {}
            _ => {}
        }
    }

    let mut badge_lines: Vec<&str> = badges.into_iter().collect();
    badge_lines.sort();

    // Always append the dx-anywhere badge at the end (using repo logo)
    let dx_anywhere_badge = "[![dx-anywhere](https://img.shields.io/badge/DX--Anywhere-CLI-1ED6FF?logo=https://raw.githubusercontent.com/dx-anywhere/dx-cli/HEAD/images/dx-logo.svg)](#)";
    if badge_lines.is_empty() {
        dx_anywhere_badge.to_string()
    } else {
        format!("{} {}", badge_lines.join(" "), dx_anywhere_badge)
    }
}

/// Upsert badges block in README.md within markers.
pub fn upsert_badges_in_readme(project_dir: &Path, badges_line: &str) -> std::io::Result<PathBuf> {
    let readme_path = project_dir.join("README.md");

    let replacement_block = format!(
        "{start}\n{badges}\n{end}\n",
        start = START_MARKER,
        badges = badges_line,
        end = END_MARKER
    );

    if readme_path.exists() {
        let mut content = fs::read_to_string(&readme_path)?;
        // Replace existing block if found
        if let (Some(start_idx), Some(end_idx)) =
            (content.find(START_MARKER), content.find(END_MARKER))
        {
            let end_idx = end_idx + END_MARKER.len();
            content.replace_range(start_idx..end_idx, &replacement_block);
        } else {
            // Insert below first H1 heading if present, else at top
            if let Some(pos) = content.find('\n') {
                // Try to find first heading starting with '#'
                let mut insert_at = 0usize;
                for (offset, line) in content.lines().enumerate() {
                    if line.trim_start().starts_with('#') {
                        // position after this line
                        let before: String = content
                            .lines()
                            .take(offset + 1)
                            .collect::<Vec<_>>()
                            .join("\n");
                        insert_at = before.len();
                        break;
                    }
                }
                if insert_at == 0 {
                    // no heading found; insert at top after first line break if any
                    insert_at = pos + 1;
                }
                content.insert_str(insert_at, &format!("\n{}\n", replacement_block));
            } else {
                // single-line file; prepend
                content = format!("{}\n\n{}\n{}", content, replacement_block, "");
            }
        }
        fs::write(&readme_path, content)?;
    } else {
        // Create a minimal README with badges
        let mut content = String::new();
        content.push_str("# Projeto\n\n");
        content.push_str(&replacement_block);
        fs::write(&readme_path, content)?;
    }

    Ok(readme_path)
}

/// Process one directory: detect services and apply badges (print or save)
pub fn process_directory(save_file: bool, project_dir: &Path) {
    let config = dev_services::detect_dependencies(project_dir);
    let mut services: Vec<String> = config.services.keys().cloned().collect();
    services.sort();

    let badges = generate_badges_markdown(&services);

    println!(
        "Badges detectados para {}:\n{}\n",
        project_dir.display(),
        badges
    );

    if save_file {
        match upsert_badges_in_readme(project_dir, &badges) {
            Ok(path) => println!("README atualizado: {}", path.display()),
            Err(e) => eprintln!(
                "Erro ao atualizar README em {}: {}",
                project_dir.display(),
                e
            ),
        }
    } else {
        println!("Execução em modo --no-save. Para salvar badges, execute: dx-cli dev-badges");
    }
}

/// Remove the badges block from README.md if present. Returns (path, removed?)
pub fn remove_badges_in_readme(project_dir: &Path) -> std::io::Result<(PathBuf, bool)> {
    let readme_path = project_dir.join("README.md");
    if !readme_path.exists() {
        println!(
            "README inexistente em {} — nada para limpar.",
            project_dir.display()
        );
        return Ok((readme_path, false));
    }

    let content = fs::read_to_string(&readme_path)?;
    let Some(start_idx) = content.find(START_MARKER) else {
        println!(
            "Nenhum bloco de badges encontrado em {}.",
            readme_path.display()
        );
        return Ok((readme_path, false));
    };
    let Some(end_start) = content.find(END_MARKER) else {
        println!(
            "Marcador inicial encontrado mas o final não existe em {} — nenhuma alteração.",
            readme_path.display()
        );
        return Ok((readme_path, false));
    };
    let end_idx = end_start + END_MARKER.len();

    // Remove the block and also trim excessive blank lines around it
    let mut new_content = String::new();
    new_content.push_str(&content[..start_idx]);
    new_content.push_str(&content[end_idx..]);

    // Collapse 3+ newlines to at most 2 for cleanliness
    let cleaned = collapse_blank_lines(&new_content);

    fs::write(&readme_path, cleaned)?;
    println!("Badges removidos de {}", readme_path.display());
    Ok((readme_path, true))
}

fn collapse_blank_lines(s: &str) -> String {
    let mut prev_blank = false;
    let mut out = String::new();
    for line in s.lines() {
        let blank = line.trim().is_empty();
        if blank && prev_blank {
            // skip extra blank line
            continue;
        }
        out.push_str(line);
        out.push('\n');
        prev_blank = blank;
    }
    out
}

/// Orchestrates cleaning for a directory
pub fn process_clean_directory(project_dir: &Path) {
    match remove_badges_in_readme(project_dir) {
        Ok((_path, removed)) => {
            if !removed {
                // nothing removed
            }
        }
        Err(e) => eprintln!("Erro ao limpar badges em {}: {}", project_dir.display(), e),
    }
}
