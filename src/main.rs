// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dx",
    version,
    about = "DX em qualquer stack",
    long_about = "Projeto: DX em qualquer stack\n\nObjetivo: criar um conjunto de padrões, toolkits e automações que reduzam atrito do primeiro commit ao deploy — em qualquer stack — com IA promovendo ciclos de feedback curtos e decisões melhores.\n\nPilares (com IA embutida):\n- Ambiente instantâneo (\"Dev Services\" universais)\n- Dev UI portátil (portal do dev)\n- Testes Contínuos & Inteligentes\n- Configuração sem dor\n- Docs vivas + Q&A no código\n- Governança leve, guardrails fortes\n- Telemetria e feedback loops curtos",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Provisionamento automático de dependências (manifesto Dev Services)
    DevServices {
        /// Ação opcional (ex.: `run`). Se omitida, gera/mostra o manifesto.
        #[command(subcommand)]
        action: Option<DevServicesAction>,
        /// Não salva o manifesto detectado como docker-compose.yml (por padrão, o manifesto é salvo)
        #[arg(long)]
        no_save: bool,
        /// Diretório raiz no qual detectar dependências e gerar .dx/docker-compose.yml (opcional; padrão: diretório atual)
        dir: Option<std::path::PathBuf>,
    },
    /// Gera e aplica badges de tecnologias detectadas no README.md, ou limpa badges existentes via subcomando `clean`
    DevBadges {
        /// Ação opcional (ex.: `clean`). Se omitida, aplica/injeta badges.
        #[command(subcommand)]
        action: Option<DevBadgesAction>,
        /// Não salva no README (apenas imprime as badges). Por padrão, salva. Apenas para a ação de aplicar.
        #[arg(long, default_value_t = false)]
        no_save: bool,
        /// Diretório alvo (padrão: diretório atual). Para `clean`, também pode ser informado após o subcomando.
        dir: Option<std::path::PathBuf>,
    },
    /// Executa testes unitários continuamente ao detectar mudanças nos arquivos
    DevTest {
        /// Diretório raiz do projeto a ser monitorado (opcional; padrão: diretório atual)
        dir: Option<std::path::PathBuf>,
    },
    /// Gerencia configurações do projeto e identifica a stack
    DevConfig {
        /// Ação opcional (ex.: `add`). Se omitida, lista configurações.
        #[command(subcommand)]
        action: Option<DevConfigAction>,
        /// Diretório raiz do projeto (opcional; padrão: diretório atual)
        dir: Option<std::path::PathBuf>,
    },
    /// Gerencia dependências de desenvolvimento do projeto
    DevDependencies {
        /// Ação opcional (ex.: `add`). Se omitida, lista dependências.
        #[command(subcommand)]
        action: Option<DevDependenciesAction>,
        /// Diretório raiz do projeto (opcional; padrão: diretório atual)
        dir: Option<std::path::PathBuf>,
    },
    /// Portal/plug-in do desenvolvedor (Dev UI)
    Portal,
    /// Testes contínuos e inteligentes (geração/execução)
    Tests,
    /// Configuração tipada com wizards em linguagem natural
    Config,
    /// Documentação viva e Q&A no código
    Docs,
    /// Governança leve com guardrails
    Governance,
    /// Limpa pastas .dx recursivamente a partir do diretório informado (ou do diretório atual se omitido)
    Clean {
        /// Diretório raiz a partir do qual limpar .dx (opcional; padrão: diretório atual)
        dir: Option<std::path::PathBuf>,
    },
    /// Analisa o projeto e resume o que o dx-cli aplicaria (todas as capabilities)
    #[command(alias = "test-stacks", hide = true)]
    #[command(alias = "doctor", hide = true)]
    Analyzer {
        /// Não salva o relatório (por padrão, o relatório é salvo)
        #[arg(long)]
        no_save: bool,
        /// Caminho para salvar o relatório (padrão: analyzer-report.md)
        #[arg(long, default_value = "analyzer-report.md")]
        report_path: String,
        /// Diretório do projeto a ser analisado (opcional; padrão: diretório atual)
        dir: Option<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
enum DevBadgesAction {
    /// Limpa os badges do README.md entre os marcadores padrão
    Clean {
        /// Diretório alvo (opcional). Se omitido, usa o diretório atual.
        dir: Option<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
enum DevServicesAction {
    /// Executa o docker compose localizado em .dx/docker-compose.yml (sobe serviços em segundo plano)
    Run {
        /// Diretório alvo (opcional). Se omitido, usa o diretório atual.
        dir: Option<std::path::PathBuf>,
    },
    /// Para (stop) os containers definidos em .dx/docker-compose.yml
    Stop {
        /// Diretório alvo (opcional). Se omitido, usa o diretório atual.
        dir: Option<std::path::PathBuf>,
    },
    /// Reinicia (restart) os containers definidos em .dx/docker-compose.yml
    Restart {
        /// Diretório alvo (opcional). Se omitido, usa o diretório atual.
        dir: Option<std::path::PathBuf>,
    },
    /// Remove (down) os containers definidos em .dx/docker-compose.yml (não remove volumes)
    Remove {
        /// Diretório alvo (opcional). Se omitido, usa o diretório atual.
        dir: Option<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
enum DevConfigAction {
    /// Lista todas as configurações
    List,
    /// Cria nova configuração
    Add {
        /// Chave da configuração
        key: String,
        /// Valor da configuração
        value: String,
    },
    /// Atualiza configuração existente
    Update {
        /// Chave da configuração
        key: String,
        /// Novo valor da configuração
        value: String,
    },
    /// Remove uma configuração
    Delete {
        /// Chave da configuração
        key: String,
    },
}

#[derive(Subcommand)]
enum DevDependenciesAction {
    /// Lista todas as dependências de desenvolvimento
    List,
    /// Adiciona uma nova dependência de desenvolvimento
    Add {
        /// Nome da dependência
        name: String,
        /// Versão (opcional)
        version: Option<String>,
    },
    /// Atualiza uma dependência específica ou todas se omitido
    Update {
        /// Nome da dependência (opcional)
        name: Option<String>,
    },
    /// Remove uma dependência de desenvolvimento
    Delete {
        /// Nome da dependência
        name: String,
    },
}


mod dev_badges;
mod dev_config;
mod dev_test;
mod dev_dependencies;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::DevServices { action, no_save, dir } => {
            match action {
                Some(DevServicesAction::Run { dir: d2 }) => cmd_dev_services_run(d2.or(dir)),
                Some(DevServicesAction::Stop { dir: d2 }) => cmd_dev_services_stop(d2.or(dir)),
                Some(DevServicesAction::Restart { dir: d2 }) => cmd_dev_services_restart(d2.or(dir)),
                Some(DevServicesAction::Remove { dir: d2 }) => cmd_dev_services_remove(d2.or(dir)),
                None => cmd_dev_services(!no_save, dir),
            }
        }
        Commands::DevBadges { action, no_save, dir } => {
            match action {
                Some(DevBadgesAction::Clean { dir: d2 }) => cmd_dev_badges_clean(d2.or(dir)),
                None => cmd_dev_badges(!no_save, dir),
            }
        }
        Commands::DevTest { dir } => dev_test::watch_and_test(dir),
        Commands::DevConfig { action, dir } => match action.unwrap_or(DevConfigAction::List) {
            DevConfigAction::List => dev_config::list(dir),
            DevConfigAction::Add { key, value } => dev_config::add(dir, key, value),
            DevConfigAction::Update { key, value } => dev_config::update(dir, key, value),
            DevConfigAction::Delete { key } => dev_config::delete(dir, key),
        },
        Commands::DevDependencies { action, dir } => match action.unwrap_or(DevDependenciesAction::List) {
            DevDependenciesAction::List => dev_dependencies::list(dir),
            DevDependenciesAction::Add { name, version } => dev_dependencies::add(dir, name, version),
            DevDependenciesAction::Update { name } => dev_dependencies::update(dir, name),
            DevDependenciesAction::Delete { name } => dev_dependencies::delete(dir, name),
        },
        Commands::Portal => cmd_portal(),
        Commands::Tests => cmd_tests(),
        Commands::Config => cmd_config(),
        Commands::Docs => cmd_docs(),
        Commands::Governance => cmd_governance(),
        Commands::Clean { dir } => cmd_clean(dir),
        Commands::Analyzer {
            no_save,
            report_path,
            dir,
        } => cmd_analyzer(!no_save, report_path, dir),
    }
}


mod dev_services;
mod telemetry;
mod report;

fn cmd_dev_services(save_file: bool, dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    // Determine target directory (provided or current)
    let target_dir = dir.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));

    // Helper: process a single project directory
    fn process_project_dir(save_file: bool, project_dir: &Path) {
        use crate::dev_services;
        use std::fs;

        // Detect dependencies
        let config = dev_services::detect_dependencies(project_dir);

        // Create .dx directory if it doesn't exist
        let dx_dir = project_dir.join(".dx");
        if save_file && !dx_dir.exists() {
            if let Err(e) = fs::create_dir_all(&dx_dir) {
                eprintln!(
                    "Erro ao criar diretório .dx em {}: {}",
                    project_dir.display(),
                    e
                );
                return;
            }
        }


        // Always print the detected dependencies
        println!(
            "Manifesto Dev Services (detectado) para {}:\n",
            project_dir.display()
        );

        if config.services.is_empty() {
            println!("Nenhuma dependência detectada no projeto atual.");
        } else {
            println!("---");
            println!("{}", config.to_yaml());

            // Handle file saving based on save_file parameter
            if save_file {
                println!("\nSalvando manifesto como .dx/docker-compose.yml...");

                match crate::telemetry::apply(project_dir) {
                    Ok(res) => {
                        println!("Arquivo docker-compose.yml criado com sucesso em:");
                        println!("{}", res.compose_path.display());
                        println!("\nPara iniciar os serviços (incluindo Telemetry), execute:");
                        println!("docker compose -f .dx/docker-compose.yml up -d");
                        println!("ou, se estiver usando a CLI legada:");
                        println!("docker-compose -f .dx/docker-compose.yml up -d");
                        println!("\nDica: você também pode rodar: dx dev-services run");
                        println!("Para parar os serviços depois: dx dev-services stop");
                        println!("Para reiniciar os serviços: dx dev-services restart");
                        println!("Para remover os containers: dx dev-services remove");

                        // Generate analyzer-style report (same as `dx analyzer`)
                        let report_path = project_dir.join(".dx").join("analyzer-report.md");
                        let report = crate::report::build_analyzer_report(project_dir, &res.config);
                        if let Some(parent) = report_path.parent() { let _ = std::fs::create_dir_all(parent); }
                        match std::fs::write(&report_path, report) {
                            Ok(_) => println!("\nRelatório (analyzer) gerado: {}", report_path.display()),
                            Err(e) => eprintln!("\nErro ao gerar relatório: {}", e),
                        }
                    }
                    Err(e) => {
                        eprintln!("Erro ao aplicar Telemetry e criar .dx/docker-compose.yml: {}", e);
                    }
                }

                println!("\nPara apenas visualizar sem salvar, execute:");
                println!("dx dev-services --no-save");
            } else {
                // Instructions for saving when using --no-save
                println!("\nPara salvar este manifesto como .dx/docker-compose.yml, execute:");
                println!("dx dev-services");
            }
        }

        println!(
            "Notas: a detecção de dependências atual é simplificada. A versão completa da ferramenta"
        );
        println!(
            "analisará código-fonte e IaC, sugerindo configurações específicas para dev local."
        );
        println!("");
    }

    // If the provided dir is the test-projects root, iterate over its immediate subdirectories
    let is_test_projects = target_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.eq_ignore_ascii_case("test-projects"))
        .unwrap_or(false);

    if is_test_projects && target_dir.is_dir() {
        println!(
            "Executando dev-services em todos os projetos dentro de: {}",
            target_dir.display()
        );
        match fs::read_dir(&target_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path: PathBuf = entry.path();
                    let Ok(ft) = entry.file_type() else { continue };
                    if ft.is_symlink() {
                        continue;
                    }
                    if ft.is_dir() {
                        println!("\n== Projeto: {} ==", path.display());
                        process_project_dir(save_file, &path);
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Erro ao listar subdiretórios de {}: {}",
                    target_dir.display(),
                    e
                );
            }
        }
        return;
    }

    // Default: process a single directory
    process_project_dir(save_file, &target_dir);
}

fn cmd_dev_services_run(dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::path::Path;
    use std::process::{Command, Stdio};

    let project_dir = dir
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    let compose_path = project_dir.join(".dx").join("docker-compose.yml");

    if !compose_path.exists() {
        eprintln!(
            "Arquivo não encontrado: {}",
            compose_path.display()
        );
        println!("Gerando manifesto automaticamente (dx dev-services) para: {}", project_dir.display());
        // Tenta gerar o manifesto e incorporar Telemetry no mesmo arquivo
        // equivalente a executar: dx dev-services <dir>
        cmd_dev_services(true, Some(project_dir.clone()));
        // Recheca se foi criado
        if !compose_path.exists() {
            eprintln!("Falha ao gerar .dx/docker-compose.yml automaticamente. Verifique mensagens acima ou execute 'dx dev-services' manualmente.");
            return;
        }
    }

    // Migração: corrigir caminhos legados para evitar erros de montagem
    // - ".dx/telemetry/" -> "telemetry/"
    // - "telemetry/" -> "./telemetry/" (força bind mount)
    if let Ok(content) = std::fs::read_to_string(&compose_path) {
        let mut fixed = content.clone();
        let mut changed = false;
        if fixed.contains(".dx/telemetry/") {
            fixed = fixed.replace(".dx/telemetry/", "telemetry/");
            changed = true;
        }
        if fixed.contains("telemetry/") && !fixed.contains("./telemetry/") {
            fixed = fixed.replace("telemetry/", "./telemetry/");
            changed = true;
        }
        if changed && fixed != content {
            match std::fs::write(&compose_path, fixed) {
                Ok(_) => println!("Ajustando caminhos de telemetry no compose (bind mounts ./telemetry)."),
                Err(e) => eprintln!("Aviso: falha ao auto-corrigir caminhos de telemetry no compose: {}", e),
            }
        }
    }

    println!("Iniciando Dev Services usando: {}", compose_path.display());

    // Prefer Docker Compose V2 (docker compose). If it fails to spawn, fallback to legacy docker-compose.
    let try_docker_compose_v2 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("up")
            .arg("-d")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    let try_docker_compose_v1 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker-compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("up")
            .arg("-d")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    match try_docker_compose_v2() {
        Ok(status) if status.success() => {
            println!("Serviços iniciados com Docker Compose (V2). Use 'docker compose ps' para ver o status.");
            return;
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker compose'. Tentando 'docker-compose' (CLI legada)...");
        }
        Err(e) => {
            eprintln!("Não foi possível executar 'docker compose': {}. Tentando 'docker-compose' (CLI legada)...", e);
        }
    }

    match try_docker_compose_v1() {
        Ok(status) if status.success() => {
            println!("Serviços iniciados com docker-compose. Use 'docker-compose ps' para ver o status.");
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker-compose'. Verifique se o Docker Desktop está instalado e em execução.");
        }
        Err(e) => {
            eprintln!("Erro ao tentar executar 'docker-compose': {}", e);
            eprintln!("Dicas:");
            eprintln!(" - Instale o Docker Desktop para Windows");
            eprintln!(" - Reabra o terminal após a instalação para atualizar o PATH");
            eprintln!(" - Teste no terminal: 'docker --version' e 'docker compose version'");
        }
    }
}

fn cmd_dev_services_stop(dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::path::Path;
    use std::process::{Command, Stdio};

    let project_dir = dir
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    let compose_path = project_dir.join(".dx").join("docker-compose.yml");

    if !compose_path.exists() {
        eprintln!(
            "Arquivo não encontrado: {}\nDica: gere o manifesto com:\n  dx dev-services\nOu especifique o diretório correto com:\n  dx dev-services stop <dir>",
            compose_path.display()
        );
        return;
    }

    println!("Parando Dev Services usando: {}", compose_path.display());

    let try_docker_compose_v2 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("stop")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    let try_docker_compose_v1 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker-compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("stop")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    match try_docker_compose_v2() {
        Ok(status) if status.success() => {
            println!("Serviços parados com Docker Compose (V2). Para iniciar novamente: 'dx dev-services run'.");
            return;
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker compose'. Tentando 'docker-compose' (CLI legada)...");
        }
        Err(e) => {
            eprintln!("Não foi possível executar 'docker compose': {}. Tentando 'docker-compose' (CLI legada)...", e);
        }
    }

    match try_docker_compose_v1() {
        Ok(status) if status.success() => {
            println!("Serviços parados com docker-compose. Para iniciar novamente: 'dx dev-services run'.");
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker-compose'. Verifique se o Docker Desktop está instalado e em execução.");
        }
        Err(e) => {
            eprintln!("Erro ao tentar executar 'docker-compose': {}", e);
            eprintln!("Dicas:");
            eprintln!(" - Instale o Docker Desktop para Windows");
            eprintln!(" - Reabra o terminal após a instalação para atualizar o PATH");
            eprintln!(" - Teste no terminal: 'docker --version' e 'docker compose version'");
        }
    }
}

fn cmd_dev_badges(save_file: bool, dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    let target_dir = dir.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));

    // Helper
    fn process_project_dir(save_file: bool, project_dir: &Path) {
        crate::dev_badges::process_directory(save_file, project_dir);
    }

    let is_test_projects = target_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.eq_ignore_ascii_case("test-projects"))
        .unwrap_or(false);

    if is_test_projects && target_dir.is_dir() {
        println!(
            "Aplicando dev-badges em todos os projetos dentro de: {}",
            target_dir.display()
        );
        match fs::read_dir(&target_dir) {
            Ok(entries) => {
                // Collect and sort subdirs for deterministic order
                let mut dirs: Vec<PathBuf> = Vec::new();
                for entry in entries.flatten() {
                    let path: PathBuf = entry.path();
                    let Ok(ft) = entry.file_type() else { continue };
                    if ft.is_symlink() { continue; }
                    if ft.is_dir() {
                        // skip hidden
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if name.starts_with('.') { continue; }
                        }
                        dirs.push(path);
                    }
                }
                dirs.sort();
                for path in dirs {
                    println!("\n== Projeto: {} ==", path.display());
                    process_project_dir(save_file, &path);
                }
            }
            Err(e) => {
                eprintln!(
                    "Erro ao listar subdiretórios de {}: {}",
                    target_dir.display(),
                    e
                );
            }
        }
        return;
    }

    process_project_dir(save_file, &target_dir);
}

fn cmd_dev_badges_clean(dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    let target_dir = dir.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));

    fn process_project_dir(project_dir: &Path) {
        crate::dev_badges::process_clean_directory(project_dir);
    }

    let is_test_projects = target_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.eq_ignore_ascii_case("test-projects"))
        .unwrap_or(false);

    if is_test_projects && target_dir.is_dir() {
        println!(
            "Limpando badges em todos os projetos dentro de: {}",
            target_dir.display()
        );
        match fs::read_dir(&target_dir) {
            Ok(entries) => {
                let mut dirs: Vec<PathBuf> = Vec::new();
                for entry in entries.flatten() {
                    let path: PathBuf = entry.path();
                    let Ok(ft) = entry.file_type() else { continue };
                    if ft.is_symlink() { continue; }
                    if ft.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if name.starts_with('.') { continue; }
                        }
                        dirs.push(path);
                    }
                }
                dirs.sort();
                for path in dirs {
                    println!("\n== Projeto: {} ==", path.display());
                    process_project_dir(&path);
                }
            }
            Err(e) => {
                eprintln!(
                    "Erro ao listar subdiretórios de {}: {}",
                    target_dir.display(),
                    e
                );
            }
        }
        return;
    }

    process_project_dir(&target_dir);
}

fn cmd_portal() {
    println!(
        "Portal do Dev (stub)\n- Gerir configs, semear dados, publicar eventos, inspecionar logs/telemetria e acionar fluxos comuns.\n- Exemplos orientados por IA: 'publique 500 eventos válidos neste tópico', 'gere massa de dados conforme este schema'."
    );
}


fn cmd_tests() {
    println!(
        "Testes Contínuos & Inteligentes (stub)\n- Geração/expansão de testes por IA (unit, contrato, property-based).\n- Fixtures realistas automáticos, priorização de falhas e explicabilidade no portal."
    );
}

fn cmd_config() {
    println!(
        "Configuração sem dor (stub)\n- Schema central tipado, wizards em linguagem natural (ex.: 'quero habilitar TLS e rodar em staging').\n- IA valida, propõe padrões e explica impacto das propriedades."
    );
}

fn cmd_docs() {
    println!(
        "Docs vivas + Q&A (stub)\n- Documentação como código, indexada e consultável via chat embutido.\n- IA referencia trechos, PRs e decisões de arquitetura; sugere golden paths."
    );
}

fn cmd_governance() {
    println!(
        "Governança leve, guardrails fortes (stub)\n- Scorecards de DX + DORA.\n- Policies automatizadas e padrões opinativos."
    );
}




/// Limpa recursivamente todas as pastas ".dx" a partir de um diretório raiz
fn clean_dx_from(root: &std::path::Path) -> (usize, Vec<String>) {
    use std::fs;
    use std::path::{Path, PathBuf};

    fn walk_and_clean(dir: &Path, removed: &mut usize, errors: &mut Vec<String>) {
        // First, attempt to remove ".dx" in this directory, if present
        let dx_here = dir.join(".dx");
        if dx_here.is_dir() {
            match fs::remove_dir_all(&dx_here) {
                Ok(_) => {
                    *removed += 1;
                    println!("Removido: {}", dx_here.display());
                }
                Err(e) => {
                    let msg = format!("Falha ao remover {}: {}", dx_here.display(), e);
                    eprintln!("{}", msg);
                    errors.push(msg);
                }
            }
        }

        // Walk children
        match fs::read_dir(dir) {
            Ok(read_dir) => {
                for entry in read_dir.flatten() {
                    let path: PathBuf = entry.path();
                    // Skip if not a directory
                    let Ok(ft) = entry.file_type() else { continue };
                    if ft.is_symlink() {
                        // avoid following symlinks
                        continue;
                    }
                    if ft.is_dir() {
                        // Recurse into subdirectory
                        walk_and_clean(&path, removed, errors);
                    }
                }
            }
            Err(e) => {
                let msg = format!("Falha ao listar {}: {}", dir.display(), e);
                eprintln!("{}", msg);
                errors.push(msg);
            }
        }
    }

    let mut removed = 0usize;
    let mut errors = Vec::new();
    walk_and_clean(root, &mut removed, &mut errors);
    (removed, errors)
}

fn cmd_clean(dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::path::Path;

    let root = dir.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));

    if !root.exists() || !root.is_dir() {
        eprintln!("Diretório inválido para limpeza: {}", root.display());
        return;
    }

    println!("Limpando pastas .dx a partir de: {}", root.display());
    let (removed, errors) = clean_dx_from(&root);

    if removed == 0 {
        println!("Nenhuma pasta .dx encontrada sob {}", root.display());
    } else {
        println!("Pasta(s) .dx removidas: {}", removed);
    }

    if !errors.is_empty() {
        eprintln!("Ocorreram {} erro(s) durante a limpeza:", errors.len());
        for e in errors {
            eprintln!("- {}", e);
        }
    }
}

fn cmd_analyzer(save_report: bool, report_path: String, dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    // Helper: generate markdown report content for a given directory and its detected config
    fn build_report(project_dir: &Path, ds_config: &dev_services::DockerComposeConfig) -> String {
        crate::report::build_analyzer_report(project_dir, ds_config)
    }

    // Helper: decide output path for a given root and desired report_path
    fn compute_output_path(root_dir: &Path, report_path: &str) -> (PathBuf, bool) {
        // returns (final_path, used_default)
        let dx_dir = root_dir.join(".dx");
        let default_name = "analyzer-report.md";
        if report_path == default_name {
            return (dx_dir.join(default_name), true);
        }
        let custom = PathBuf::from(report_path);
        if custom.is_absolute() {
            // absolute; let caller decide if multi; here we just return it
            (custom, false)
        } else {
            (root_dir.join(custom), false)
        }
    }

    // Helper: check if a directory looks like a project root by presence of marker files
    fn is_project_root(dir: &Path) -> bool {
        let markers = [
            "Cargo.toml",
            "package.json",
            "requirements.txt",
            "pyproject.toml",
            "setup.py",
            "pom.xml",
            "build.gradle",
            "Gemfile",
            "go.mod",
            "composer.json",
        ];
        markers.iter().any(|m| dir.join(m).is_file())
    }

    // Helper: list candidate subprojects under a directory following directory rules
    fn list_subprojects(root: &Path) -> Vec<PathBuf> {
        let skip = [
            ".git", ".github", ".idea", ".vscode", ".dx", "node_modules", "target", "build", "dist", "vendor",
        ];
        let mut subs = Vec::new();
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() { continue; }
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.') { continue; }
                if skip.iter().any(|s| s.eq_ignore_ascii_case(&name)) { continue; }
                if is_project_root(&path) {
                    subs.push(path);
                }
            }
        }
        subs
    }

    // Ensure the analyzed directory's .gitignore contains an entry to ignore .dx; create if needed
    fn ensure_gitignore_has_dx(dir: &Path) {
        use std::fs::OpenOptions;
        use std::io::Write;

        let gi_path = dir.join(".gitignore");
        match fs::read_to_string(&gi_path) {
            Ok(content) => {
                let mut has = false;
                for line in content.lines() {
                    let t = line.trim();
                    if t == ".dx" || t == "/.dx" || t == ".dx/" { has = true; break; }
                }
                if !has {
                    let mut file = match OpenOptions::new().create(true).append(true).open(&gi_path) {
                        Ok(f) => f,
                        Err(_) => return,
                    };
                    // Ensure previous content ends with newline to avoid gluing
                    if !content.is_empty() && !content.ends_with(['\n', '\r']) {
                        let _ = writeln!(file);
                    }
                    let _ = writeln!(file, ".dx");
                }
            }
            Err(_) => {
                // No .gitignore: create one with .dx
                let _ = fs::write(&gi_path, ".dx\n");
            }
        }
    }

    let cwd = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let project_dir: PathBuf = if let Some(provided) = dir {
        if provided.is_absolute() { provided } else { cwd.join(provided) }
    } else {
        cwd.clone()
    };

    if !project_dir.exists() || !project_dir.is_dir() {
        eprintln!("Diretório inválido para análise: {}", project_dir.display());
        return;
    }

    println!("dx analyzer\n");
    println!("Analisando o projeto em: {}\n", project_dir.display());

    // If the provided directory contains multiple recognizable subprojects, produce per-directory reports
    let subprojects = list_subprojects(&project_dir);
    let multi = !subprojects.is_empty();

    if multi {
        println!("Detectamos múltiplos projetos dentro de {}. Gerando relatórios por diretório...", project_dir.display());
        let mut count_ok = 0usize;
        for sub in &subprojects {
            // Ensure .gitignore ignores .dx in each subproject
            ensure_gitignore_has_dx(sub);
            println!("\n--- Projeto: {} ---", sub.display());
            let ds_config = dev_services::detect_dependencies(sub);

            // Print a brief console summary per subproject
            if ds_config.services.is_empty() {
                println!("Nenhuma dependência de serviços detectada.");
            } else {
                let services: Vec<_> = ds_config.services.keys().cloned().collect();
                println!("Dependências detectadas: {:?}", services);
            }

            if save_report {
                // Compute output path; if absolute custom path is given, avoid overwriting by falling back to default per-dir
                let (mut out_path, used_default) = compute_output_path(sub, &report_path);
                if out_path.is_absolute() && !used_default && report_path != "analyzer-report.md" {
                    eprintln!("Aviso: caminho absoluto customizado informado para múltiplos relatórios. Usando padrão por diretório em {}.", sub.display());
                    out_path = sub.join(".dx").join("analyzer-report.md");
                }
                if let Some(parent) = out_path.parent() { let _ = fs::create_dir_all(parent); }
                let report = build_report(sub, &ds_config);
                match fs::write(&out_path, report) {
                    Ok(_) => { println!("Relatório salvo em: {}", out_path.display()); count_ok += 1; }
                    Err(e) => eprintln!("Erro ao salvar relatório em {}: {}", out_path.display(), e),
                }
            }
        }
        if !save_report {
            println!("\nPara salvar os relatórios, execute sem --no-save ou forneça --report-path (relativo). Cada relatório será salvo no .dx de cada projeto.");
        } else {
            println!("\nRelatórios gerados: {}/{}", count_ok, subprojects.len());
        }
        return;
    }

    // Single-project behavior (existing flow)
    // Ensure .gitignore ignores .dx in this project
    ensure_gitignore_has_dx(&project_dir);
    let ds_config = dev_services::detect_dependencies(&project_dir);
    println!("=== Dev Services ===");
    if ds_config.services.is_empty() {
        println!("Nenhuma dependência de serviços detectada.");
        println!("Sugestão: adicione variáveis/.env ou dependências para Postgres, Redis, Kafka (Redpanda), MongoDB, Flink, etc.\n");
    } else {
        let services: Vec<_> = ds_config.services.keys().cloned().collect();
        println!("Dependências detectadas: {:?}", services);
        println!("\nManifesto gerado (docker-compose.yml):\n");
        let yaml = ds_config.to_yaml();
        println!("{}", yaml);
    }

    // 2) Dev Badges (stub): quais badges poderíamos aplicar
    println!("\n=== Dev Badges ===");
    if ds_config.services.is_empty() {
        println!("Sem badges a aplicar no momento.");
    } else {
        println!("Badges sugeridas com base nas dependências detectadas: {}",
                 ds_config.services.keys().cloned().collect::<Vec<_>>().join(", "));
        println!("Use: dx dev-badges (ou dev-badges clean)");
    }

    // 3) Portal (stub)
    println!("\n=== Portal (Dev UI) ===");
    println!("Integrações e operações do desenvolvedor em um só lugar. Em breve: automações e plugins.\nUse: dx portal");

    // 4) Testes (stub)
    println!("\n=== Testes Contínuos & Inteligentes ===");
    println!("Geração/execução de testes assistidos por IA (futuro).\nUse: dx tests");

    // 5) Configuração (stub)
    println!("\n=== Configuração ===");
    println!("Configuração tipada com wizards em linguagem natural.\nUse: dx config");

    // 6) Documentação (stub)
    println!("\n=== Documentação ===");
    println!("Docs vivas + Q&A no código (busca conversacional).\nUse: dx docs");

    // 7) Governança (stub)
    println!("\n=== Governança ===");
    println!("Guardrails, scorecards e automações de qualidade.\nUse: dx governance");

    // 8) Telemetria (stub)
    println!("\n=== Telemetria ===");
    println!("Observabilidade e feedback loops curtos por padrão.\nUse: dx dev-services");

    if save_report {
        let (final_path, _used_default) = compute_output_path(&project_dir, &report_path);
        // Ensure parent exists
        if let Some(parent) = final_path.parent() { let _ = fs::create_dir_all(parent); }
        let report = build_report(&project_dir, &ds_config);
        match fs::write(&final_path, report) {
            Ok(_) => println!("\nRelatório salvo em: {}", final_path.display()),
            Err(e) => eprintln!("\nErro ao salvar relatório: {}", e),
        }
    } else {
        println!("\nPara salvar este relatório, execute sem --no-save ou use --report-path");
    }
}


fn cmd_dev_services_restart(dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::path::Path;
    use std::process::{Command, Stdio};

    let project_dir = dir
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    let compose_path = project_dir.join(".dx").join("docker-compose.yml");

    if !compose_path.exists() {
        eprintln!(
            "Arquivo não encontrado: {}\nDica: gere o manifesto com:\n  dx-cli dev-services\nOu especifique o diretório correto com:\n  dx-cli dev-services restart <dir>",
            compose_path.display()
        );
        return;
    }

    println!("Reiniciando Dev Services usando: {}", compose_path.display());

    let try_docker_compose_v2 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("restart")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    let try_docker_compose_v1 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker-compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("restart")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    match try_docker_compose_v2() {
        Ok(status) if status.success() => {
            println!("Serviços reiniciados com Docker Compose (V2). Use 'docker compose ps' para ver o status.");
            return;
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker compose'. Tentando 'docker-compose' (CLI legada)...");
        }
        Err(e) => {
            eprintln!("Não foi possível executar 'docker compose': {}. Tentando 'docker-compose' (CLI legada)...", e);
        }
    }

    match try_docker_compose_v1() {
        Ok(status) if status.success() => {
            println!("Serviços reiniciados com docker-compose. Use 'docker-compose ps' para ver o status.");
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker-compose'. Verifique se o Docker Desktop está instalado e em execução.");
        }
        Err(e) => {
            eprintln!("Erro ao tentar executar 'docker-compose': {}", e);
            eprintln!("Dicas:");
            eprintln!(" - Instale o Docker Desktop para Windows");
            eprintln!(" - Reabra o terminal após a instalação para atualizar o PATH");
            eprintln!(" - Teste no terminal: 'docker --version' e 'docker compose version'");
        }
    }
}


fn cmd_dev_services_remove(dir: Option<std::path::PathBuf>) {
    use std::env;
    use std::path::Path;
    use std::process::{Command, Stdio};

    let project_dir = dir
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()));
    let compose_path = project_dir.join(".dx").join("docker-compose.yml");

    if !compose_path.exists() {
        eprintln!(
            "Arquivo não encontrado: {}\nDica: gere o manifesto com:\n  dx-cli dev-services\nOu especifique o diretório correto com:\n  dx-cli dev-services remove <dir>",
            compose_path.display()
        );
        return;
    }

    println!("Removendo containers de Dev Services usando: {}", compose_path.display());

    let try_docker_compose_v2 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("down")
            .arg("-v")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    let try_docker_compose_v1 = || -> std::io::Result<std::process::ExitStatus> {
        Command::new("docker-compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("down")
            .arg("-v")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    match try_docker_compose_v2() {
        Ok(status) if status.success() => {
            println!("Containers e volumes removidos com Docker Compose (V2). Para iniciar novamente: 'dx-cli dev-services run'.");
            return;
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker compose'. Tentando 'docker-compose' (CLI legada)...");
        }
        Err(e) => {
            eprintln!("Não foi possível executar 'docker compose': {}. Tentando 'docker-compose' (CLI legada)...", e);
        }
    }

    match try_docker_compose_v1() {
        Ok(status) if status.success() => {
            println!("Containers e volumes removidos com docker-compose. Para iniciar novamente: 'dx-cli dev-services run'.");
        }
        Ok(_status) => {
            eprintln!("Falha ao executar 'docker-compose'. Verifique se o Docker Desktop está instalado e em execução.");
        }
        Err(e) => {
            eprintln!("Erro ao tentar executar 'docker-compose': {}", e);
            eprintln!("Dicas:");
            eprintln!(" - Instale o Docker Desktop para Windows");
            eprintln!(" - Reabra o terminal após a instalação para atualizar o PATH");
            eprintln!(" - Teste no terminal: 'docker --version' e 'docker compose version'");
        }
    }
}
