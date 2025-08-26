// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors

use crate::dev_services::{DockerComposeConfig, DockerService, create_docker_compose_file};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TelemetryResult {
    pub compose_path: PathBuf,
    pub config: DockerComposeConfig,
}

pub fn apply(project_dir: &Path) -> std::io::Result<TelemetryResult> {
    let dx_dir = project_dir.join(".dx");
    let telemetry_dir = dx_dir.join("telemetry");
    let grafana_dir = telemetry_dir.join("grafana");
    let grafana_prov_ds = grafana_dir.join("provisioning").join("datasources");
    let grafana_prov_dash = grafana_dir.join("provisioning").join("dashboards");
    let grafana_dash_dir = grafana_dir.join("dashboards");
    let prometheus_dir = telemetry_dir.join("prometheus");
    let tempo_dir = telemetry_dir.join("tempo");

    // Ensure directories
    fs::create_dir_all(&dx_dir)?;
    fs::create_dir_all(&grafana_prov_ds)?;
    fs::create_dir_all(&grafana_prov_dash)?;
    fs::create_dir_all(&grafana_dash_dir)?;
    fs::create_dir_all(&prometheus_dir)?;
    fs::create_dir_all(&tempo_dir)?;

    // Write Grafana provisioning: datasources
    let datasources_yaml = grafana_datasources_yaml();
    fs::write(grafana_prov_ds.join("datasources.yaml"), datasources_yaml)?;

    // Write Grafana provisioning: dashboards
    let dashboards_yaml = grafana_dashboards_yaml();
    fs::write(grafana_prov_dash.join("dashboards.yaml"), dashboards_yaml)?;

    // Write Prometheus config
    let prometheus_yaml = prometheus_config_yaml();
    fs::write(prometheus_dir.join("prometheus.yml"), prometheus_yaml)?;

    // Write OTel Collector config
    let otel_cfg = telemetry_dir.join("otel-collector-config.yaml");
    let otel_yaml = otel_collector_config_yaml();
    fs::write(&otel_cfg, otel_yaml)?;

    // Write Tempo config (storage backend + receivers)
    let tempo_cfg = tempo_dir.join("tempo.yaml");
    let tempo_yaml = tempo_config_yaml();
    fs::write(&tempo_cfg, tempo_yaml)?;

    // Detect language/framework and add a simple dashboard
    let (lang, framework) = detect_language_and_framework(project_dir);
    let dash = simple_dashboard_json(&lang, framework.as_deref());
    fs::write(
        grafana_dash_dir.join(format!("{}-overview.json", lang.to_lowercase())),
        dash,
    )?;

    // Build a docker-compose for telemetry and merge into the main dev-services compose
    // Start from detected dev services (if any)
    let mut base = crate::dev_services::detect_dependencies(project_dir);
    let telemetry_cfg = build_telemetry_compose();
    for (name, svc) in telemetry_cfg.services.into_iter() {
        base.add_service(&name, svc);
    }

    let compose_path = dx_dir.join("docker-compose.yml");
    create_docker_compose_file(&base, &compose_path)?;

    Ok(TelemetryResult {
        compose_path,
        config: base,
    })
}

fn build_telemetry_compose() -> DockerComposeConfig {
    let mut cfg = DockerComposeConfig::new();

    // Loki
    cfg.add_service(
        "loki",
        DockerService {
            image: "grafana/loki:2.9.6".to_string(),
            env: HashMap::new(),
            ports: vec![3100],
            volumes: vec!["loki-data:/loki".to_string()],
            command: None,
        },
    );

    // Tempo
    cfg.add_service(
        "tempo",
        DockerService {
            image: "grafana/tempo:2.5.0".to_string(),
            env: HashMap::new(),
            ports: vec![3200],
            volumes: vec![
                format!("{}:/etc/tempo.yaml", rel_bind("telemetry/tempo/tempo.yaml")),
                "tempo-data:/var/tempo".to_string(),
            ],
            command: Some("-config.file=/etc/tempo.yaml".to_string()),
        },
    );

    // Prometheus
    cfg.add_service(
        "prometheus",
        DockerService {
            image: "prom/prometheus:latest".to_string(),
            env: HashMap::new(),
            ports: vec![9090],
            volumes: vec![
                format!(
                    "{}:/etc/prometheus/prometheus.yml",
                    rel_bind("telemetry/prometheus/prometheus.yml")
                ),
                "prom-data:/prometheus".to_string(),
            ],
            command: None,
        },
    );

    // Grafana
    cfg.add_service(
        "grafana",
        DockerService {
            image: "grafana/grafana:latest".to_string(),
            env: {
                let mut e = HashMap::new();
                e.insert("GF_AUTH_ANONYMOUS_ENABLED".to_string(), "true".to_string());
                e.insert(
                    "GF_AUTH_ANONYMOUS_ORG_ROLE".to_string(),
                    "Admin".to_string(),
                );
                e
            },
            ports: vec![3000],
            volumes: vec![
                format!(
                    "{}:/etc/grafana/provisioning/datasources",
                    rel_bind("telemetry/grafana/provisioning/datasources")
                ),
                format!(
                    "{}:/etc/grafana/provisioning/dashboards",
                    rel_bind("telemetry/grafana/provisioning/dashboards")
                ),
                format!(
                    "{}:/var/lib/grafana/dashboards",
                    rel_bind("telemetry/grafana/dashboards")
                ),
                "grafana-storage:/var/lib/grafana".to_string(),
            ],
            command: None,
        },
    );

    // OpenTelemetry Collector
    cfg.add_service(
        "otel-collector",
        DockerService {
            image: "otel/opentelemetry-collector-contrib:latest".to_string(),
            env: HashMap::new(),
            ports: vec![4317, 4318, 8889],
            volumes: vec![format!(
                "{}:/etc/otel-collector-config.yaml",
                rel_bind("telemetry/otel-collector-config.yaml")
            )],
            command: Some("--config=/etc/otel-collector-config.yaml".to_string()),
        },
    );

    cfg
}

fn rel_bind(p: &str) -> String {
    // Ensure forward slashes and a leading ./ so Docker Compose treats it as a bind mount
    let mut s = p.replace('\\', "/");
    if !s.starts_with("./") && !s.starts_with('/') {
        s = format!("./{}", s);
    }
    s
}

fn grafana_datasources_yaml() -> String {
    // Provision three datasources: Prometheus, Loki, Tempo
    let s = r#"apiVersion: 1
datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
  - name: Loki
    type: loki
    access: proxy
    url: http://loki:3100
  - name: Tempo
    type: tempo
    access: proxy
    url: http://tempo:3200
"#;
    s.to_string()
}

fn grafana_dashboards_yaml() -> String {
    let s = r#"apiVersion: 1
providers:
  - name: 'Default'
    orgId: 1
    folder: ''
    type: file
    disableDeletion: false
    editable: true
    updateIntervalSeconds: 30
    options:
      path: /var/lib/grafana/dashboards
"#;
    s.to_string()
}

fn prometheus_config_yaml() -> String {
    let s = r#"global:
  scrape_interval: 30s
scrape_configs:
  - job_name: 'otel-collector'
    static_configs:
      - targets: ['otel-collector:8889']
"#;
    s.to_string()
}

fn tempo_config_yaml() -> String {
    // Minimal Tempo single-binary config with local storage and explicit OTLP receiver endpoints
    let s = r#"server:
  http_listen_port: 3200
compactor:
  compaction:
    block_retention: 24h
distributor:
  receivers:
    otlp:
      protocols:
        http:
          endpoint: 0.0.0.0:4318
        grpc:
          endpoint: 0.0.0.0:4317
ingester:
  lifecycler:
    address: 127.0.0.1
    ring:
      kvstore:
        store: inmemory
      replication_factor: 1
  trace_idle_period: 10s
  max_block_bytes: 1048576
  max_block_duration: 10m
storage:
  trace:
    backend: local
    local:
      path: /var/tempo/traces
    wal:
      path: /var/tempo/wal
"#;
    s.to_string()
}

fn otel_collector_config_yaml() -> String {
    // Expose Prometheus exporter at 0.0.0.0:8889; receive OTLP on 4317/4318; export
    // metrics to Prometheus (scraped), logs to Loki via OTLP HTTP, traces to Tempo via OTLP gRPC
    let s = r#"receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
      http:
        endpoint: 0.0.0.0:4318
exporters:
  prometheus:
    endpoint: 0.0.0.0:8889
  otlphttp/loki:
    endpoint: http://loki:3100/otlp
  otlp/tempo:
    endpoint: tempo:4317
    tls:
      insecure: true
processors:
  batch: {}
  memory_limiter:
    check_interval: 1s
    limit_mib: 200
    spike_limit_mib: 100
service:
  pipelines:
    metrics:
      receivers: [otlp]
      processors: [memory_limiter, batch]
      exporters: [prometheus]
    logs:
      receivers: [otlp]
      processors: [memory_limiter, batch]
      exporters: [otlphttp/loki]
    traces:
      receivers: [otlp]
      processors: [memory_limiter, batch]
      exporters: [otlp/tempo]
"#;
    s.to_string()
}

fn detect_language_and_framework(project_dir: &Path) -> (String, Option<String>) {
    // Very simple heuristics
    let p = project_dir;
    if p.join("Cargo.toml").exists() {
        return ("Rust".into(), None);
    }
    if p.join("package.json").exists() {
        // Try to detect common frameworks by files
        let fw = if p.join("next.config.js").exists() || p.join("next.config.ts").exists() {
            Some("Next.js".to_string())
        } else if p.join("nuxt.config.js").exists() {
            Some("Nuxt".to_string())
        } else if p.join("nest-cli.json").exists() {
            Some("NestJS".to_string())
        } else {
            Some("Node.js".to_string())
        };
        return ("JavaScript".into(), fw);
    }
    if p.join("pyproject.toml").exists() || p.join("requirements.txt").exists() {
        let fw = if p.join("manage.py").exists() {
            Some("Django".to_string())
        } else {
            None
        };
        return ("Python".into(), fw);
    }
    if p.join("pom.xml").exists() || p.join("build.gradle").exists() {
        return ("Java".into(), None);
    }
    if p.join("Gemfile").exists() {
        return ("Ruby".into(), None);
    }
    if p.join("go.mod").exists() {
        return ("Go".into(), None);
    }
    if p.join("composer.json").exists() {
        return ("PHP".into(), None);
    }
    ("General".into(), None)
}

fn simple_dashboard_json(language: &str, framework: Option<&str>) -> String {
    // A minimal Grafana dashboard JSON skeleton with Loki/Tempo/Prometheus hints
    // We avoid Rust's format! braces by using a placeholder replacement.
    let title = match framework {
        Some(fw) => format!("{} ({}) Overview", language, fw),
        None => format!("{} Overview", language),
    };
    let template = r#"{
  "annotations": {
    "list": [{
      "builtIn": 1,
      "datasource": "-- Grafana --",
      "type": "dashboard"
    }]
  },
  "editable": true,
  "fiscalYearStartMonth": 0,
  "graphTooltip": 0,
  "panels": [
    {
      "type": "timeseries",
      "title": "CPU (sample)",
      "datasource": "Prometheus",
      "targets": [{"expr": "process_cpu_seconds_total"}],
      "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
    },
    {
      "type": "logs",
      "title": "Recent Logs",
      "datasource": "Loki",
      "targets": [{"expr": "{job=~\".*\"}"}],
      "gridPos": {"h": 8, "w": 24, "x": 0, "y": 8}
    }
  ],
  "schemaVersion": 39,
  "title": "__TITLE__",
  "version": 1
}"#;
    template.replace("__TITLE__", &title)
}
