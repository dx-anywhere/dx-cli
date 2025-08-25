// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors

use crate::dev_services::{DockerComposeConfig, DockerService};
use std::path::Path;

fn linkify_image(image: &str) -> String {
    // Turn an image ref like "postgres:16-alpine" or "grafana/grafana:latest" or
    // "ghcr.io/org/app:tag" into a Markdown link to its registry page.
    let name = image.split(':').next().unwrap_or(image);
    let parts: Vec<&str> = name.split('/').collect();
    let url = match parts.as_slice() {
        // Docker Hub library images (no namespace)
        [single] => format!("https://hub.docker.com/_/{}", single),
        // Namespaced or registry-qualified
        [first, rest @ ..] => {
            let rest_path = rest.join("/");
            if first.contains('.') {
                match *first {
                    "ghcr.io" => format!("https://ghcr.io/{}", rest_path),
                    "quay.io" => format!("https://quay.io/repository/{}", rest_path),
                    "gcr.io" => format!("https://gcr.io/{}", rest_path),
                    _ => format!("https://{}/{}", first, rest_path),
                }
            } else {
                // Docker Hub with namespace
                format!("https://hub.docker.com/r/{}/{}", first, rest_path)
            }
        }
        _ => "https://hub.docker.com".to_string(),
    };
    format!("[{}]({})", image, url)
}

/// Build the rich analyzer-style markdown report for a given project directory
/// and the detected DockerComposeConfig. This is shared by `analyzer` and
/// `dev-services` so that the report content is identical.
pub fn build_analyzer_report(project_dir: &Path, ds_config: &DockerComposeConfig) -> String {
    use std::collections::HashSet;
    let mut report = String::new();

    // Header with identity and quick badges
    report.push_str("# dx-cli _analyzer_\n\n");
    report.push_str(&format!("Projeto: {}\n\n", project_dir.display()));
    report.push_str("[![dx-anywhere](https://img.shields.io/badge/DX--Anywhere-CLI-1ED6FF?logo=data:image/svg+xml;base64,aHR0cHM6Ly9yYXcuZ2l0aHVidXNlcmNvbnRlbnQuY29tL2R4LWFueXdoZXJlL2R4LWNsaS9yZWZzL2hlYWRzL21haW4vaW1hZ2VzL2R4LWxvZ28uc3Zn)](#) ");
    report.push_str("[![Report](https://img.shields.io/badge/Report-Markdown-informational)](#) ");
    report.push_str("[![Platform](https://img.shields.io/badge/Platform-Windows%20|%20macOS%20|%20Linux-green)](#)\n\n");

    // Intro/callout
    report.push_str("> ℹ️ Este relatório resume o que o dx-cli aplicaria ao seu projeto: Dev Services, badges e próximas ações.\n\n");

    // Table of contents
    report.push_str("## Tabela de Conteúdos\n");
    report.push_str("- [Resumo](#resumo)\n");
    report.push_str("- [Dev Services](#dev-services)\n");
    report.push_str("- [Badges para README.md](#badges-para-readmemd)\n");
    report.push_str("- [Próximas Ações](#próximas-ações)\n");
    report.push_str("- [Outras Capabilities](#outras-capabilities)\n\n");

    // Summary section
    report.push_str("## Resumo\n\n");
    let svc_count = ds_config.services.len();
    if svc_count == 0 {
        report.push_str("- 🚫 Nenhuma dependência de serviço detectada\n");
        report.push_str("- 💡 Dica: adicione variáveis/.env ou dependências (Postgres, Redis, Kafka/Redpanda, MongoDB, Flink, etc.)\n\n");
    } else {
        report.push_str(&format!("- ✅ Serviços detectados: {}\n", svc_count));
        let mut names: Vec<_> = ds_config.services.keys().cloned().collect();
        names.sort();
        report.push_str(&format!("- 🧩 Lista: {}\n\n", names.join(", ")));
    }

    // Dev Services section
    report.push_str("## Dev Services\n\n");
    if ds_config.services.is_empty() {
        report.push_str("Nenhuma dependência detectada.\n\n");
    } else {
        report.push_str("Serviços detectados:\n");
        for (name, _svc) in &ds_config.services {
            report.push_str(&format!("- {}\n", name));
        }

        // Services overview table
        report.push_str("\n### Visão geral dos serviços\n\n");
        report.push_str("| Serviço | Imagem | Portas | Volumes | Credenciais/Info |\n");
        report.push_str("|--------|--------|--------|---------|------------------|\n");
        let mut entries: Vec<_> = ds_config.services.iter().collect();
        entries.sort_by(|a,b| a.0.cmp(b.0));
        for (name, svc) in entries {
            let ports_md = if svc.ports.is_empty() {
                "-".to_string()
            } else {
                svc.ports
                    .iter()
                    .map(|p| format!("[{}](http://localhost:{})", p, p))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let vols = if svc.volumes.is_empty() { "-".to_string() } else { svc.volumes.len().to_string() };
            let info = service_info(name, svc);
            let image_link = linkify_image(&svc.image);
            report.push_str(&format!("| {} | {} | {} | {} | {} |\n", name, image_link, ports_md, vols, info));
        }

        // Proposed YAML (collapsible)
        report.push_str("\n### docker-compose.yaml proposto\n\n");
        report.push_str("<details>\n");
        report.push_str("<summary>Mostrar YAML</summary>\n\n");
        report.push_str("```yaml\n");
        report.push_str(&ds_config.to_yaml());
        report.push_str("\n```\n");
        report.push_str("</details>\n\n");

        // Tip callout
        report.push_str("> 💡 Dica: ajuste portas/volumes conforme seu ambiente. Com Docker Compose v2, use `docker compose` em vez de `docker-compose`.\n\n");
    }

    // Badges section for README injection
    report.push_str("## Badges para README.md\n\n");
    report.push_str("Abaixo você vê as badges renderizadas. Em seguida, há um bloco colapsável com o Markdown para copiar e colar entre os marcadores no seu README.md.\n\n");
    // Build badges
    let mut badges: HashSet<&str> = HashSet::new();
    let keys: HashSet<String> = ds_config.services.keys().cloned().collect();
    for k in &keys {
        let kl = k.to_lowercase();
        match kl.as_str() {
            "postgres" => { badges.insert("[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Dev_Service-blue?logo=postgresql)](#)"); },
            "mysql" => { badges.insert("[![MySQL](https://img.shields.io/badge/MySQL-Dev_Service-blue?logo=mysql)](#)"); },
            "redis" => { badges.insert("[![Redis](https://img.shields.io/badge/Redis-Dev_Service-red?logo=redis)](#)"); },
            "mongodb" => { badges.insert("[![MongoDB](https://img.shields.io/badge/MongoDB-Dev_Service-green?logo=mongodb)](#)"); },
            "kafka" => { badges.insert("[![Kafka](https://img.shields.io/badge/Kafka-Dev_Service-black?logo=apachekafka)](#)"); },
            "kafka-ui" => { /* skip explicit UI badge */ },
            "jobmanager" | "taskmanager" => { badges.insert("[![Apache Flink](https://img.shields.io/badge/Flink-Dev_Service-orange?logo=apacheflink)](#)"); },
            _ => {}
        }
    }
    let mut badge_lines: Vec<&str> = badges.into_iter().collect();
    badge_lines.sort();
    // Always append the dx-anywhere badge at the end (using repo logo)
    let dx_anywhere_badge = "[![dx-anywhere](https://img.shields.io/badge/DX--Anywhere-CLI-1ED6FF?logo=data:image/svg+xml;base64,aHR0cHM6Ly9yYXcuZ2l0aHVidXNlcmNvbnRlbnQuY29tL2R4LWFueXdoZXJlL2R4LWNsaS9yZWZzL2hlYWRzL21haW4vaW1hZ2VzL2R4LWxvZ28uc3Zn)](#)";
    let rendered_line = if badge_lines.is_empty() {
        dx_anywhere_badge.to_string()
    } else {
        format!("{} {}", badge_lines.join(" "), dx_anywhere_badge)
    };
    // Rendered badges line
    report.push_str(&rendered_line);
    report.push_str("\n\n");
    // Collapsible code block for README injection
    report.push_str("<details>\n");
    report.push_str("<summary>Mostrar bloco de badges (Markdown)</summary>\n\n");
    report.push_str("```md\n");
    report.push_str("<!-- dx-cli:badges:start -->\n");
    report.push_str(&rendered_line);
    report.push_str("\n");
    report.push_str("<!-- dx-cli:badges:end -->\n");
    report.push_str("```\n\n");
    report.push_str("</details>\n\n");

    // Next steps
    report.push_str("## Próximas Ações\n\n");
    report.push_str("- 🧪 Visualizar ajuda da CLI: `dx --help`\n");
    report.push_str("- 🧱 Gerar/Salvar Dev Services: `dx dev-services`\n");
    report.push_str("- 🏷️ Aplicar badges: `dx dev-badges` (ou `dx dev-badges clean`)\n");
    report.push_str("- 🩺 Reexecutar análise: `dx analyzer`\n\n");

    report.push_str("## Outras Capabilities\n\n");
    report.push_str("- Dev Badges: aplicar badges das tecnologias detectadas (dx dev-badges)\n");
    report.push_str("- Portal: Dev UI com integrações e operações (dx portal)\n");
    report.push_str("- Testes: geração/execução assistidas (dx tests)\n");
    report.push_str("- Config: wizards e config tipada (dx config)\n");
    report.push_str("- Docs: documentação viva + Q&A (dx docs)\n");
    report.push_str("- Governança: guardrails e scorecards (dx governance)\n");
    report.push_str("- Telemetria: observabilidade por padrão (inclusa no dev-services)\n\n");

    // Footer
    report.push_str("---\n");
    report.push_str("Relatório gerado pelo dx-cli.\n");

    report
}

fn service_info(name: &str, svc: &DockerService) -> String {
    let n = name.to_lowercase();
    // Convenience closure to fetch env var
    let env = |k: &str| svc.env.get(k).cloned();

    match n.as_str() {
        // Databases
        "postgres" => {
            let user = env("POSTGRES_USER").unwrap_or_else(|| "postgres".to_string());
            let pass = env("POSTGRES_PASSWORD").unwrap_or_else(|| "example".to_string());
            let db = env("POSTGRES_DB").unwrap_or_else(|| "app".to_string());
            format!("user: {}, pass: {}, db: {}, url: postgres://{}:{}@localhost:5432/{}", user, pass, db, user, pass, db)
        }
        "mysql" | "mariadb" => {
            let user = "root".to_string();
            let pass = env("MARIADB_ROOT_PASSWORD").or_else(|| env("MYSQL_ROOT_PASSWORD")).unwrap_or_else(|| "example".to_string());
            let db = env("MARIADB_DATABASE").or_else(|| env("MYSQL_DATABASE")).unwrap_or_else(|| "app".to_string());
            format!("user: {}, pass: {}, db: {}, url: mysql://{}:{}@localhost:3306/{}", user, pass, db, user, pass, db)
        }
        "mongodb" => {
            let user = env("MONGO_INITDB_ROOT_USERNAME").unwrap_or_else(|| "root".to_string());
            let pass = env("MONGO_INITDB_ROOT_PASSWORD").unwrap_or_else(|| "example".to_string());
            format!("user: {}, pass: {}, url: mongodb://{}:{}@localhost:27017", user, pass, user, pass)
        }
        "redis" => {
            // If REDIS_PASSWORD present, report it, otherwise default: no auth
            if let Some(p) = env("REDIS_PASSWORD") { format!("senha: {} (requirepass habilitado)", p) } else { "sem senha (default)".to_string() }
        }
        // Messaging / Streaming
        "kafka" => {
            // Redpanda default advertised host 29092
            "Bootstrap: localhost:29092".to_string()
        }
        "kafka-ui" => {
            "UI: http://localhost:9093".to_string()
        }
        // Flink
        "jobmanager" => {
            "Flink UI: http://localhost:8081".to_string()
        }
        "taskmanager" => {
            "Seguido pelo JobManager (sem UI)".to_string()
        }
        // Observability stack
        "grafana" => {
            let anon = svc
                .env
                .get("GF_AUTH_ANONYMOUS_ENABLED")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            if anon {
                "login anônimo (Admin)".to_string()
            } else {
                "credenciais padrão configuráveis".to_string()
            }
        }
        "prometheus" => {
            "scrape: otel-collector:8889".to_string()
        }
        "loki" => {
            "push: http://localhost:3100/loki/api/v1/push".to_string()
        }
        "tempo" => {
            "OTLP gRPC: 4317, HTTP: 4318".to_string()
        }
        "otel-collector" => {
            "OTLP HTTP: 4318 | gRPC: 4317 | Prom (metrics): 8889".to_string()
        }
        _ => "-".to_string(),
    }
}
