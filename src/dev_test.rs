// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors

use std::{
    fmt,
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};

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

    fn test_command(self, dir: &Path) -> Option<(String, Vec<String>)> {
        match self {
            Stack::Rust => Some(("cargo".into(), vec!["test".into()])),
            Stack::Node => Some(("npm".into(), vec!["test".into()])),
            Stack::Python => Some(("python".into(), vec!["-m".into(), "pytest".into()])),
            Stack::Go => Some(("go".into(), vec!["test".into(), "./...".into()])),
            Stack::JavaMaven => Some(("mvn".into(), vec!["test".into()])),
            Stack::JavaGradle => {
                if dir.join("gradlew").exists() {
                    Some(("./gradlew".into(), vec!["test".into()]))
                } else {
                    Some(("gradle".into(), vec!["test".into()]))
                }
            }
            Stack::Unknown => None,
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

fn run_tests(dir: &Path, cmd: &str, args: &[String]) {
    println!("> Executando testes: {} {:?}", cmd, args);
    match Command::new(cmd).args(args).current_dir(dir).status() {
        Ok(status) if status.success() => println!("> Testes concluídos com sucesso"),
        Ok(status) => println!("> Testes falharam (status {status})"),
        Err(e) => eprintln!("Erro ao executar comando de teste: {e}"),
    }
}

fn should_ignore(path: &Path) -> bool {
    path.components().any(|comp| {
        matches!(
            comp.as_os_str().to_str(),
            Some(c) if c.starts_with('.') || c == "target" || c == "node_modules"
        )
    })
}

/// Watch files in `dir` and re-run unit tests on changes.
/// Detects the project stack automatically to choose the test command.
pub fn watch_and_test(dir: Option<PathBuf>) {
    let project_dir =
        dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let stack = Stack::detect(&project_dir);
    let Some((cmd, args)) = stack.test_command(&project_dir) else {
        eprintln!("Stack não reconhecida em {}", project_dir.display());
        return;
    };

    println!("Stack detectada: {}", stack);
    println!(
        "Monitorando alterações em {} (Ctrl-C para sair)",
        project_dir.display()
    );

    run_tests(&project_dir, &cmd, &args);

    let (tx, rx) = channel();

    let mut watcher = recommended_watcher(move |res| {
        tx.send(res).ok();
    })
    .expect("não foi possível iniciar watcher");

    watcher
        .watch(&project_dir, RecursiveMode::Recursive)
        .expect("não foi possível observar diretório");

    const DEBOUNCE_MS: u64 = 500;
    let mut last_run = Instant::now();

    for res in rx {
        match res {
            Ok(event) => {
                if matches!(
                    event.kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                ) {
                    if event.paths.iter().any(|p| !should_ignore(p))
                        && last_run.elapsed() >= Duration::from_millis(DEBOUNCE_MS)
                    {
                        last_run = Instant::now();
                        println!("Alterações detectadas. Executando testes...");
                        run_tests(&project_dir, &cmd, &args);
                    }
                }
            }
            Err(e) => eprintln!("Erro do watcher: {e}"),
        }
    }
}
