// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct DockerService {
    pub image: String,
    pub env: HashMap<String, String>,
    pub ports: Vec<u16>,
    pub volumes: Vec<String>,
    pub command: Option<String>,
}

#[derive(Default)]
pub struct DockerComposeConfig {
    pub version: String,
    pub services: HashMap<String, DockerService>,
}

impl DockerComposeConfig {
    pub fn new() -> Self {
        DockerComposeConfig {
            version: "3.8".to_string(),
            services: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, name: &str, service: DockerService) {
        self.services.insert(name.to_string(), service);
    }

    pub fn to_yaml(&self) -> String {
        let mut yaml = format!("version: '{}'\nservices:\n", self.version);

        // Collect all defined volumes
        let mut volumes = Vec::new();

        for (name, service) in &self.services {
            yaml.push_str(&format!("  {}:\n", name));
            yaml.push_str(&format!("    image: {}\n", service.image));

            if let Some(cmd) = &service.command {
                yaml.push_str(&format!("    command: {}\n", cmd));
            }

            if !service.env.is_empty() {
                yaml.push_str("    environment:\n");
                for (key, value) in &service.env {
                    let force_block = key == "FLINK_PROPERTIES";
                    if force_block || value.contains('\n') {
                        yaml.push_str(&format!("      {}: |\n", key));
                        for line in value.lines() {
                            yaml.push_str(&format!("        {}\n", line));
                        }
                    } else {
                        yaml.push_str(&format!("      {}: {}\n", key, value));
                    }
                }
            }

            if !service.ports.is_empty() {
                yaml.push_str("    ports:\n");
                for port in &service.ports {
                    yaml.push_str(&format!("      - '{}:{}'\n", port, port));
                }
            }

            if !service.volumes.is_empty() {
                yaml.push_str("    volumes:\n");
                for volume in &service.volumes {
                    yaml.push_str(&format!("      - {}\n", volume));

                    // Extract the volume name (before the colon)
                    if let Some(volume_name) = volume.split(':').next() {
                        if !volume_name.contains('/') && !volume_name.contains('\\') {
                            // Likely a named volume, not a bind mount
                            volumes.push(volume_name);
                        }
                    }
                }
            }
        }

        // Add volumes section if there are any named volumes
        if !volumes.is_empty() {
            yaml.push_str("\nvolumes:\n");
            for volume in volumes.iter().collect::<std::collections::HashSet<_>>() {
                yaml.push_str(&format!("  {}:\n", volume));
            }
        }

        yaml
    }
}

pub fn detect_dependencies(project_dir: &Path) -> DockerComposeConfig {
    let mut config = DockerComposeConfig::new();

    // Check for common dependencies in project files
    if has_postgres_dependency(project_dir) {
        let mut env = HashMap::new();
        env.insert("POSTGRES_PASSWORD".to_string(), "example".to_string());
        env.insert("POSTGRES_DB".to_string(), "app".to_string());

        config.add_service(
            "postgres",
            DockerService {
                image: "postgres:16-alpine".to_string(),
                env,
                ports: vec![5432],
                volumes: vec!["postgres-data:/var/lib/postgresql/data".to_string()],
                command: None,
            },
        );
    }

    if has_mysql_dependency(project_dir) {
        let mut env = HashMap::new();
        env.insert("MARIADB_ROOT_PASSWORD".to_string(), "example".to_string());
        env.insert("MARIADB_DATABASE".to_string(), "app".to_string());

        // Use MariaDB for a fully open-source, lighter MySQL-compatible server
        config.add_service(
            "mysql",
            DockerService {
                image: "mariadb:11".to_string(),
                env,
                ports: vec![3306],
                volumes: vec!["mariadb-data:/var/lib/mysql".to_string()],
                command: None,
            },
        );
    }

    if has_kafka_dependency(project_dir) {
        // Use Redpanda: Kafka API-compatible, lightweight, no-cost for local dev
        let env = HashMap::new();
        let redpanda_cmd = "redpanda start --overprovisioned --smp 1 --memory 512M --reserve-memory 0M --node-id 0 --check=false --kafka-addr PLAINTEXT://0.0.0.0:9092,PLAINTEXT_HOST://0.0.0.0:29092 --advertise-kafka-addr PLAINTEXT://kafka:9092,PLAINTEXT_HOST://localhost:29092".to_string();

        config.add_service(
            "kafka",
            DockerService {
                image: "redpandadata/redpanda:latest".to_string(),
                env,
                ports: vec![9092, 29092],
                volumes: vec!["redpanda-data:/var/lib/redpanda/data".to_string()],
                command: Some(redpanda_cmd),
            },
        );

        // Add Kafka UI for local inspection when Kafka is present
        let mut ui_env = HashMap::new();
        ui_env.insert("KAFKA_CLUSTERS_0_NAME".to_string(), "local".to_string());
        ui_env.insert(
            "KAFKA_CLUSTERS_0_BOOTSTRAPSERVERS".to_string(),
            "kafka:9092".to_string(),
        );
        // Change default UI port to 9093 to avoid conflicts and match N:N mapping
        ui_env.insert("SERVER_PORT".to_string(), "9093".to_string());
        config.add_service(
            "kafka-ui",
            DockerService {
                image: "provectuslabs/kafka-ui:latest".to_string(),
                env: ui_env,
                ports: vec![9093],
                volumes: vec![],
                command: None,
            },
        );
    }

    if has_redis_dependency(project_dir) {
        config.add_service(
            "redis",
            DockerService {
                image: "redis:alpine".to_string(),
                env: HashMap::new(),
                ports: vec![6379],
                volumes: vec!["redis-data:/data".to_string()],
                command: None,
            },
        );
    }

    if has_mongodb_dependency(project_dir) {
        let mut env = HashMap::new();
        env.insert("MONGO_INITDB_ROOT_USERNAME".to_string(), "root".to_string());
        env.insert(
            "MONGO_INITDB_ROOT_PASSWORD".to_string(),
            "example".to_string(),
        );

        config.add_service(
            "mongodb",
            DockerService {
                image: "mongo:7.0".to_string(),
                env,
                ports: vec![27017],
                volumes: vec!["mongodb-data:/data/db".to_string()],
                command: None,
            },
        );
    }

    if has_flink_dependency(project_dir) {
        // Apache Flink dependencies typically require multiple services

        // JobManager service
        let mut jobmanager_env = HashMap::new();
        jobmanager_env.insert(
            "FLINK_PROPERTIES".to_string(),
            "jobmanager.rpc.address: jobmanager".to_string(),
        );

        config.add_service(
            "jobmanager",
            DockerService {
                image: "apache/flink:latest".to_string(),
                env: jobmanager_env,
                ports: vec![8081], // UI port
                volumes: vec!["flink-data:/opt/flink/data".to_string()],
                command: None,
            },
        );

        // TaskManager service
        let mut taskmanager_env = HashMap::new();
        taskmanager_env.insert(
            "FLINK_PROPERTIES".to_string(),
            "jobmanager.rpc.address: jobmanager\ntaskmanager.numberOfTaskSlots: 1".to_string(),
        );

        config.add_service(
            "taskmanager",
            DockerService {
                image: "apache/flink:latest".to_string(),
                env: taskmanager_env,
                ports: vec![],
                volumes: vec!["flink-data:/opt/flink/data".to_string()],
                command: None,
            },
        );
    }

    // Add volumes section if there are services with volumes
    let has_volumes = config.services.values().any(|s| !s.volumes.is_empty());
    if has_volumes {
        // In a real implementation, we would add volumes section to the YAML
        // This is handled in the to_yaml method for simplicity
    }

    config
}

fn has_postgres_dependency(project_dir: &Path) -> bool {
    // Search for common Postgres-related strings
    search_for_dependency(
        project_dir,
        &[
            "postgres",
            "pg",
            "postgresql",
            "psycopg",
            "POSTGRES_URL",
            "DATABASE_URL",
        ],
    )
}

fn has_mysql_dependency(project_dir: &Path) -> bool {
    // Search for common MySQL-related strings
    search_for_dependency(
        project_dir,
        &[
            "mysql",
            "mariadb",
            "innodb",
            "MYSQL_",
            "DB_CONNECTION=mysql",
        ],
    )
}

fn has_kafka_dependency(project_dir: &Path) -> bool {
    // Search for Kafka-related strings
    search_for_dependency(
        project_dir,
        &["kafka", "KAFKA_BROKERS", "kafka-go", "spring-kafka"],
    )
}

fn has_redis_dependency(project_dir: &Path) -> bool {
    // Search for Redis-related strings
    search_for_dependency(
        project_dir,
        &["redis", "REDIS_URL", "REDIS_HOST", "redis-client", "predis"],
    )
}

fn has_mongodb_dependency(project_dir: &Path) -> bool {
    // Search for MongoDB-related strings
    search_for_dependency(
        project_dir,
        &["mongodb", "mongo", "MONGO_URI", "mongoose", "mongo-driver"],
    )
}

fn has_flink_dependency(project_dir: &Path) -> bool {
    // Search for Apache Flink-related strings
    search_for_dependency(
        project_dir,
        &[
            "flink",
            "org.apache.flink",
            "flink-connector",
            "StreamExecutionEnvironment",
            "DataStream",
        ],
    )
}

fn search_for_dependency(project_dir: &Path, keywords: &[&str]) -> bool {
    // Check configuration files and package manager files first
    if check_config_files(project_dir, keywords) {
        return true;
    }

    // Then do a more thorough recursive scan of source directories
    recursive_scan_directories(project_dir, keywords)
}

// Check common configuration files and package manager files
fn check_config_files(project_dir: &Path, keywords: &[&str]) -> bool {
    // Check common .env files first (used across many languages)
    let env_path = project_dir.join(".env");
    if check_file_for_keywords(&env_path, keywords) {
        return true;
    }

    // Rust - Cargo.toml
    let cargo_toml_path = project_dir.join("Cargo.toml");
    if check_file_for_keywords(&cargo_toml_path, keywords) {
        return true;
    }

    // Node.js - package.json
    let package_json_path = project_dir.join("package.json");
    if check_file_for_keywords(&package_json_path, keywords) {
        return true;
    }

    // Python - requirements.txt, setup.py, pyproject.toml
    let requirements_path = project_dir.join("requirements.txt");
    if check_file_for_keywords(&requirements_path, keywords) {
        return true;
    }

    let setup_py_path = project_dir.join("setup.py");
    if check_file_for_keywords(&setup_py_path, keywords) {
        return true;
    }

    let pyproject_path = project_dir.join("pyproject.toml");
    if check_file_for_keywords(&pyproject_path, keywords) {
        return true;
    }

    // Java - pom.xml, build.gradle
    let pom_xml_path = project_dir.join("pom.xml");
    if check_file_for_keywords(&pom_xml_path, keywords) {
        return true;
    }

    let gradle_path = project_dir.join("build.gradle");
    if check_file_for_keywords(&gradle_path, keywords) {
        return true;
    }

    // Ruby - Gemfile
    let gemfile_path = project_dir.join("Gemfile");
    if check_file_for_keywords(&gemfile_path, keywords) {
        return true;
    }

    // Go - go.mod
    let go_mod_path = project_dir.join("go.mod");
    if check_file_for_keywords(&go_mod_path, keywords) {
        return true;
    }

    // PHP - composer.json
    let composer_json_path = project_dir.join("composer.json");
    if check_file_for_keywords(&composer_json_path, keywords) {
        return true;
    }

    // Check application-specific config files
    // Java Spring - application.properties, application.yml
    let spring_properties_path = project_dir.join("src/main/resources/application.properties");
    if check_file_for_keywords(&spring_properties_path, keywords) {
        return true;
    }

    let spring_yml_path = project_dir.join("src/main/resources/application.yml");
    if check_file_for_keywords(&spring_yml_path, keywords) {
        return true;
    }

    // Ruby on Rails - config/database.yml
    let rails_db_path = project_dir.join("config/database.yml");
    if check_file_for_keywords(&rails_db_path, keywords) {
        return true;
    }

    // Python Django - settings.py
    let django_settings_paths = vec![
        project_dir.join("settings.py"),
        project_dir.join("config/settings.py"),
        project_dir.join("app/settings.py"),
    ];

    for path in django_settings_paths {
        if check_file_for_keywords(&path, keywords) {
            return true;
        }
    }

    // No matches found in config files
    false
}

// Recursively scan directories for source files that might indicate dependencies
fn recursive_scan_directories(project_dir: &Path, keywords: &[&str]) -> bool {
    // Define common source directories to scan
    let source_dirs = vec![
        "src",      // Generic source directory
        "app",      // Common for many frameworks
        "lib",      // Ruby, PHP
        "internal", // Go
        "tests", "test",   // Test directories might have dependencies
        "config", // Configuration files
    ];

    // Define file extensions to check
    let file_extensions = vec![
        ".rs", // Rust
        ".js", ".jsx", ".ts", ".tsx", // JavaScript/TypeScript
        ".py",  // Python
        ".java", ".kt",  // Java, Kotlin
        ".rb",  // Ruby
        ".go",  // Go
        ".php", // PHP
        ".yml", ".yaml", // YAML config
        ".json", // JSON config
        ".xml",  // XML config
        ".toml", // TOML config
        ".ini", ".conf", ".cfg", // Other config formats
    ];

    // Skip directories that are commonly large and not useful for dependency detection
    let skip_dirs = vec![
        "node_modules",
        "target",
        "build",
        "dist",
        "vendor",
        ".git",
        ".github",
        ".idea",
        ".vscode",
    ];

    // Recursively walk the directory
    if let Ok(entries) = fs::read_dir(project_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Skip if path is in the skip list
            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");

                if skip_dirs.iter().any(|skip| skip == &dir_name) {
                    continue;
                }

                // Recursively check subdirectories
                if recursive_scan_directories(&path, keywords) {
                    return true;
                }

                // Continue if this directory isn't in our source_dirs list
                // Only for top-level directories - we check all subdirectories
                if path.parent() == Some(project_dir)
                    && !source_dirs.iter().any(|src| path.ends_with(src))
                {
                    continue;
                }
            } else if path.is_file() {
                // Check if the file has an extension we're interested in
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if file_extensions.iter().any(|e| e.ends_with(ext)) {
                        if check_file_for_keywords(&path, keywords) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

// Helper function to check a file for keywords
fn check_file_for_keywords(file_path: &Path, keywords: &[&str]) -> bool {
    if file_path.exists() {
        if let Ok(content) = fs::read_to_string(file_path) {
            let content_lower = content.to_lowercase();
            for keyword in keywords {
                if content_lower.contains(&keyword.to_lowercase()) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn create_docker_compose_file(
    config: &DockerComposeConfig,
    output_path: &Path,
) -> std::io::Result<()> {
    let yaml = config.to_yaml();
    fs::write(output_path, yaml)
}
