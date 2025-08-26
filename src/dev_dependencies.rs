// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors

use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use toml_edit::{Document, value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stack {
    Node,
    Rust,
    Python,
    Go,
    Maven,
    Gradle,
    Php,
    Ruby,
    Unknown,
}

impl Stack {
    fn detect(dir: &Path) -> Stack {
        if dir.join("package.json").exists() {
            Stack::Node
        } else if dir.join("Cargo.toml").exists() {
            Stack::Rust
        } else if dir.join("requirements-dev.txt").exists()
            || dir.join("requirements.txt").exists()
            || dir.join("pyproject.toml").exists()
        {
            Stack::Python
        } else if dir.join("go.mod").exists() {
            Stack::Go
        } else if dir.join("pom.xml").exists() {
            Stack::Maven
        } else if dir.join("build.gradle").exists() || dir.join("build.gradle.kts").exists() {
            Stack::Gradle
        } else if dir.join("composer.json").exists() {
            Stack::Php
        } else if dir.join("Gemfile").exists() {
            Stack::Ruby
        } else {
            Stack::Unknown
        }
    }
}

fn project_dir(dir: Option<PathBuf>) -> PathBuf {
    dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

#[derive(Debug, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub update_command: String,
    pub url: String,
}

impl DependencyInfo {
    pub fn link(&self) -> String {
        format!("[{}]({})", self.name, self.url)
    }
}

pub fn list(dir: Option<PathBuf>) {
    let project_dir = project_dir(dir);
    match Stack::detect(&project_dir) {
        Stack::Node => list_node(&project_dir),
        Stack::Rust => list_rust(&project_dir),
        Stack::Python => list_python(&project_dir),
        Stack::Go => list_go(&project_dir),
        Stack::Maven => list_maven(&project_dir),
        Stack::Gradle => list_gradle(&project_dir),
        Stack::Php => list_php(&project_dir),
        Stack::Ruby => list_ruby(&project_dir),
        Stack::Unknown => println!("Stack não suportada ou não detectada."),
    }
}

pub fn add(dir: Option<PathBuf>, name: String, version: Option<String>) {
    let project_dir = project_dir(dir);
    match Stack::detect(&project_dir) {
        Stack::Node => add_node(&project_dir, name, version),
        Stack::Rust => add_rust(&project_dir, name, version),
        Stack::Python => add_python(&project_dir, name, version),
        Stack::Php => add_php(&project_dir, name, version),
        Stack::Go => add_go(&project_dir, name, version),
        Stack::Maven => add_maven(&project_dir, name, version),
        Stack::Gradle => add_gradle(&project_dir, name, version),
        Stack::Ruby => add_ruby(&project_dir, name, version),
        Stack::Unknown => println!("Stack não suportada ou não detectada."),
    }
}

pub fn update(dir: Option<PathBuf>, name: Option<String>) {
    let project_dir = project_dir(dir);
    match Stack::detect(&project_dir) {
        Stack::Node => update_node(&project_dir, name),
        Stack::Rust => update_rust(&project_dir, name),
        Stack::Python => update_python(&project_dir, name),
        Stack::Php => update_php(&project_dir, name),
        Stack::Go => update_go(&project_dir, name),
        Stack::Maven => update_maven(&project_dir, name),
        Stack::Gradle => update_gradle(&project_dir, name),
        Stack::Ruby => update_ruby(&project_dir, name),
        Stack::Unknown => println!("Stack não suportada ou não detectada."),
    }
}

pub fn delete(dir: Option<PathBuf>, name: String) {
    let project_dir = project_dir(dir);
    match Stack::detect(&project_dir) {
        Stack::Node => delete_node(&project_dir, name),
        Stack::Rust => delete_rust(&project_dir, name),
        Stack::Python => delete_python(&project_dir, name),
        Stack::Php => delete_php(&project_dir, name),
        Stack::Go => delete_go(&project_dir, name),
        Stack::Maven => delete_maven(&project_dir, name),
        Stack::Gradle => delete_gradle(&project_dir, name),
        Stack::Ruby => delete_ruby(&project_dir, name),
        Stack::Unknown => println!("Stack não suportada ou não detectada."),
    }
}

pub fn get_dependencies(dir: &Path) -> io::Result<Vec<DependencyInfo>> {
    match Stack::detect(dir) {
        Stack::Node => Ok(get_node_dependencies(dir)),
        Stack::Rust => Ok(get_rust_dependencies(dir)),
        Stack::Python => Ok(get_python_dependencies(dir)),
        Stack::Go => Ok(get_go_dependencies(dir)),
        Stack::Maven => Ok(get_maven_dependencies(dir)),
        Stack::Gradle => Ok(get_gradle_dependencies(dir)),
        Stack::Php => Ok(get_php_dependencies(dir)),
        Stack::Ruby => Ok(get_ruby_dependencies(dir)),
        Stack::Unknown => Ok(Vec::new()),
    }
}

// Node helpers
fn node_package_json(path: &Path) -> PathBuf {
    path.join("package.json")
}

fn load_package_json(path: &Path) -> Value {
    let data = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Value::Object(Default::default()))
}

fn save_package_json(path: &Path, v: &Value) {
    if let Ok(data) = serde_json::to_string_pretty(v) {
        if let Err(e) = fs::write(path, data) {
            eprintln!("Erro ao salvar package.json: {e}");
        }
    }
}

fn list_node(dir: &Path) {
    let path = node_package_json(dir);
    let v = load_package_json(&path);
    if let Some(obj) = v.get("devDependencies").and_then(|d| d.as_object()) {
        for (k, v) in obj {
            if let Some(ver) = v.as_str() {
                println!("- {k} = {ver}");
            }
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn add_node(dir: &Path, name: String, version: Option<String>) {
    let path = node_package_json(dir);
    let mut v = load_package_json(&path);
    let obj = v
        .as_object_mut()
        .unwrap()
        .entry("devDependencies")
        .or_insert_with(|| Value::Object(Default::default()));
    if let Some(map) = obj.as_object_mut() {
        map.insert(name.clone(), Value::String(version.unwrap_or("*".into())));
        save_package_json(&path, &v);
        println!("Dependência '{name}' adicionada.");
    }
}

fn fetch_latest_node(name: &str) -> Option<String> {
    let url = format!("https://registry.npmjs.org/{}/latest", name);
    reqwest::blocking::get(url)
        .ok()?
        .json::<Value>()
        .ok()?
        .get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn update_node(dir: &Path, name: Option<String>) {
    let path = node_package_json(dir);
    let mut v = load_package_json(&path);
    let obj = v
        .as_object_mut()
        .unwrap()
        .entry("devDependencies")
        .or_insert_with(|| Value::Object(Default::default()));
    if let Some(map) = obj.as_object_mut() {
        if let Some(n) = name {
            if let Some(latest) = fetch_latest_node(&n) {
                map.insert(n.clone(), Value::String(latest));
                println!("Dependência '{n}' atualizada.");
            }
        } else {
            for (k, val) in map.iter_mut() {
                if let Some(latest) = fetch_latest_node(k) {
                    *val = Value::String(latest);
                }
            }
            println!("Todas as dependências atualizadas.");
        }
        save_package_json(&path, &v);
    }
}

fn delete_node(dir: &Path, name: String) {
    let path = node_package_json(dir);
    let mut v = load_package_json(&path);
    if let Some(obj) = v
        .as_object_mut()
        .and_then(|o| o.get_mut("devDependencies"))
        .and_then(|d| d.as_object_mut())
    {
        if obj.remove(&name).is_some() {
            println!("Dependência '{name}' removida.");
        }
    }
    save_package_json(&path, &v);
}

fn get_node_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = node_package_json(dir);
    let v = load_package_json(&path);
    let mut deps = Vec::new();
    if let Some(obj) = v.get("devDependencies").and_then(|d| d.as_object()) {
        for (k, v) in obj {
            if let Some(ver) = v.as_str() {
                let latest = fetch_latest_node(k);
                deps.push(DependencyInfo {
                    name: k.clone(),
                    current_version: ver.to_string(),
                    latest_version: latest.clone(),
                    update_command: format!("npm install {}@latest -D", k),
                    url: format!("https://www.npmjs.com/package/{}", k),
                });
            }
        }
    }
    deps
}

// Rust helpers
fn cargo_toml(path: &Path) -> PathBuf {
    path.join("Cargo.toml")
}

fn load_cargo_toml(path: &Path) -> Document {
    let data = fs::read_to_string(path).unwrap_or_default();
    data.parse::<Document>().unwrap_or_default()
}

fn save_cargo_toml(path: &Path, doc: &Document) {
    if let Err(e) = fs::write(path, doc.to_string()) {
        eprintln!("Erro ao salvar Cargo.toml: {e}");
    }
}

fn list_rust(dir: &Path) {
    let path = cargo_toml(dir);
    let doc = load_cargo_toml(&path);
    if let Some(table) = doc.get("dev-dependencies").and_then(|t| t.as_table()) {
        for (k, v) in table.iter() {
            println!(
                "- {} = {}",
                k,
                v.as_value().map(|v| v.to_string()).unwrap_or_default()
            );
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn add_rust(dir: &Path, name: String, version: Option<String>) {
    let path = cargo_toml(dir);
    let mut doc = load_cargo_toml(&path);
    let tbl = doc
        .as_table_mut()
        .entry("dev-dependencies")
        .or_insert(toml_edit::Item::Table(Default::default()))
        .as_table_mut()
        .unwrap();
    tbl.insert(name.clone(), value(version.unwrap_or("*".into())));
    save_cargo_toml(&path, &doc);
    println!("Dependência '{name}' adicionada.");
}

fn fetch_latest_crate(name: &str) -> Option<String> {
    let url = format!("https://crates.io/api/v1/crates/{}", name);
    reqwest::blocking::get(url)
        .ok()?
        .json::<Value>()
        .ok()?
        .get("crate")
        .and_then(|c| c.get("max_stable_version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn update_rust(dir: &Path, name: Option<String>) {
    let path = cargo_toml(dir);
    let mut doc = load_cargo_toml(&path);
    if let Some(table) = doc
        .get_mut("dev-dependencies")
        .and_then(|t| t.as_table_mut())
    {
        if let Some(n) = name {
            if let Some(latest) = fetch_latest_crate(&n) {
                table.insert(&n, value(latest));
                println!("Dependência '{n}' atualizada.");
            }
        } else {
            for (k, item) in table.iter_mut() {
                if let Some(latest) = fetch_latest_crate(k) {
                    *item.value_mut() = value(latest);
                }
            }
            println!("Todas as dependências atualizadas.");
        }
    }
    save_cargo_toml(&path, &doc);
}

fn delete_rust(dir: &Path, name: String) {
    let path = cargo_toml(dir);
    let mut doc = load_cargo_toml(&path);
    if let Some(table) = doc
        .get_mut("dev-dependencies")
        .and_then(|t| t.as_table_mut())
    {
        table.remove(&name);
        println!("Dependência '{name}' removida.");
    }
    save_cargo_toml(&path, &doc);
}

fn get_rust_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = cargo_toml(dir);
    let doc = load_cargo_toml(&path);
    let mut deps = Vec::new();
    if let Some(table) = doc.get("dev-dependencies").and_then(|t| t.as_table()) {
        for (k, v) in table.iter() {
            let ver = v.as_value().map(|v| v.to_string()).unwrap_or_default();
            let latest = fetch_latest_crate(k);
            deps.push(DependencyInfo {
                name: k.to_string(),
                current_version: ver.clone(),
                latest_version: latest.clone(),
                update_command: format!(
                    "cargo update -p {} --precise {}",
                    k,
                    latest.clone().unwrap_or_default()
                ),
                url: format!("https://crates.io/crates/{}", k),
            });
        }
    }
    deps
}

// Python helpers
fn requirements_path(dir: &Path) -> PathBuf {
    if dir.join("requirements-dev.txt").exists() {
        dir.join("requirements-dev.txt")
    } else {
        dir.join("requirements.txt")
    }
}

fn parse_requirements(content: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((name, ver)) = line.split_once("==") {
            map.insert(name.trim().to_string(), ver.trim().to_string());
        } else {
            map.insert(line.to_string(), "*".into());
        }
    }
    map
}

fn write_requirements(path: &Path, map: &BTreeMap<String, String>) {
    let mut out = String::new();
    for (k, v) in map {
        if v == "*" {
            out.push_str(&format!("{}\n", k));
        } else {
            out.push_str(&format!("{}=={}\n", k, v));
        }
    }
    if let Err(e) = fs::write(path, out) {
        eprintln!("Erro ao salvar requirements: {e}");
    }
}

fn list_python(dir: &Path) {
    let path = requirements_path(dir);
    if let Ok(data) = fs::read_to_string(&path) {
        let map = parse_requirements(&data);
        if map.is_empty() {
            println!("Nenhuma dependência encontrada.");
        } else {
            for (k, v) in map {
                println!("- {} = {}", k, v);
            }
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn add_python(dir: &Path, name: String, version: Option<String>) {
    let path = requirements_path(dir);
    let mut map = if let Ok(data) = fs::read_to_string(&path) {
        parse_requirements(&data)
    } else {
        BTreeMap::new()
    };
    map.insert(name.clone(), version.unwrap_or("*".into()));
    write_requirements(&path, &map);
    println!("Dependência '{name}' adicionada.");
}

fn fetch_latest_pypi(name: &str) -> Option<String> {
    let url = format!("https://pypi.org/pypi/{}/json", name);
    reqwest::blocking::get(url)
        .ok()?
        .json::<Value>()
        .ok()?
        .get("info")
        .and_then(|i| i.get("version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn update_python(dir: &Path, name: Option<String>) {
    let path = requirements_path(dir);
    if let Ok(data) = fs::read_to_string(&path) {
        let mut map = parse_requirements(&data);
        if let Some(n) = name {
            if let Some(latest) = fetch_latest_pypi(&n) {
                map.insert(n.clone(), latest);
                println!("Dependência '{n}' atualizada.");
            }
        } else {
            for (k, v) in map.iter_mut() {
                if let Some(latest) = fetch_latest_pypi(k) {
                    *v = latest;
                }
            }
            println!("Todas as dependências atualizadas.");
        }
        write_requirements(&path, &map);
    }
}

fn delete_python(dir: &Path, name: String) {
    let path = requirements_path(dir);
    if let Ok(data) = fs::read_to_string(&path) {
        let mut map = parse_requirements(&data);
        map.remove(&name);
        write_requirements(&path, &map);
        println!("Dependência '{name}' removida.");
    }
}

fn get_python_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = requirements_path(dir);
    let mut deps = Vec::new();
    if let Ok(data) = fs::read_to_string(&path) {
        let map = parse_requirements(&data);
        for (k, v) in map {
            let latest = fetch_latest_pypi(&k);
            deps.push(DependencyInfo {
                name: k.clone(),
                current_version: v.clone(),
                latest_version: latest.clone(),
                update_command: format!("pip install -U {}", k),
                url: format!("https://pypi.org/project/{}/", k),
            });
        }
    }
    deps
}

// Go helpers (listing only)
fn go_mod_path(dir: &Path) -> PathBuf {
    dir.join("go.mod")
}

fn parse_go_mod(data: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let mut in_block = false;
    for line in data.lines() {
        let line = line.trim();
        if line.starts_with("require (") {
            in_block = true;
            continue;
        }
        if in_block && line.starts_with(')') {
            in_block = false;
            continue;
        }
        if line.starts_with("require") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                map.insert(parts[1].to_string(), parts[2].to_string());
            }
            continue;
        }
        if in_block && !line.is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                map.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }
    map
}

fn list_go(dir: &Path) {
    let path = go_mod_path(dir);
    if let Ok(data) = fs::read_to_string(&path) {
        let map = parse_go_mod(&data);
        if map.is_empty() {
            println!("Nenhuma dependência encontrada.");
        } else {
            for (k, v) in map {
                println!("- {} = {}", k, v);
            }
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn fetch_latest_go(name: &str) -> Option<String> {
    let url = format!("https://proxy.golang.org/{}/@latest", name);
    reqwest::blocking::get(url)
        .ok()?
        .json::<Value>()
        .ok()?
        .get("Version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn add_go(_dir: &Path, _name: String, _version: Option<String>) {
    println!("Operação não suportada para Go.");
}

fn update_go(_dir: &Path, _name: Option<String>) {
    println!("Operação não suportada para Go.");
}

fn delete_go(_dir: &Path, _name: String) {
    println!("Operação não suportada para Go.");
}

fn get_go_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = go_mod_path(dir);
    let mut deps = Vec::new();
    if let Ok(data) = fs::read_to_string(&path) {
        let map = parse_go_mod(&data);
        for (k, v) in map {
            let latest = fetch_latest_go(&k);
            deps.push(DependencyInfo {
                name: k.clone(),
                current_version: v.clone(),
                latest_version: latest.clone(),
                update_command: format!("go get {}@latest", k),
                url: format!("https://pkg.go.dev/{}", k),
            });
        }
    }
    deps
}

// Maven helpers
fn pom_xml_path(dir: &Path) -> PathBuf {
    dir.join("pom.xml")
}

fn parse_maven_deps(data: &str) -> Vec<(String, String, String)> {
    let mut deps = Vec::new();
    let mut rest = data;
    while let Some(start) = rest.find("<dependency>") {
        rest = &rest[start + "<dependency>".len()..];
        if let Some(end) = rest.find("</dependency>") {
            let block = &rest[..end];
            rest = &rest[end + "</dependency>".len()..];
            if block.contains("<scope>test</scope>") {
                let group = extract_between(block, "<groupId>", "</groupId>").unwrap_or_default();
                let artifact =
                    extract_between(block, "<artifactId>", "</artifactId>").unwrap_or_default();
                let version = extract_between(block, "<version>", "</version>").unwrap_or_default();
                deps.push((group.to_string(), artifact.to_string(), version.to_string()));
            }
        } else {
            break;
        }
    }
    deps
}

fn extract_between<'a>(hay: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let s = hay.find(start)? + start.len();
    let e = hay[s..].find(end)? + s;
    Some(&hay[s..e])
}

fn list_maven(dir: &Path) {
    let path = pom_xml_path(dir);
    if let Ok(data) = fs::read_to_string(&path) {
        let deps = parse_maven_deps(&data);
        if deps.is_empty() {
            println!("Nenhuma dependência encontrada.");
        } else {
            for (g, a, v) in deps {
                println!("- {}:{} = {}", g, a, v);
            }
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn fetch_latest_maven(group: &str, artifact: &str) -> Option<String> {
    let path = group.replace('.', "/");
    let url = format!(
        "https://repo1.maven.org/maven2/{}/{}/maven-metadata.xml",
        path, artifact
    );
    let text = reqwest::blocking::get(url).ok()?.text().ok()?;
    extract_between(&text, "<latest>", "</latest>")
        .or_else(|| extract_between(&text, "<release>", "</release>"))
        .map(|s| s.to_string())
}

fn add_maven(_dir: &Path, _name: String, _version: Option<String>) {
    println!("Operação não suportada para Maven.");
}

fn update_maven(_dir: &Path, _name: Option<String>) {
    println!("Operação não suportada para Maven.");
}

fn delete_maven(_dir: &Path, _name: String) {
    println!("Operação não suportada para Maven.");
}

fn get_maven_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = pom_xml_path(dir);
    let mut deps = Vec::new();
    if let Ok(data) = fs::read_to_string(&path) {
        for (g, a, v) in parse_maven_deps(&data) {
            let latest = fetch_latest_maven(&g, &a);
            let name = format!("{}:{}", g, a);
            deps.push(DependencyInfo {
                name: name.clone(),
                current_version: v.clone(),
                latest_version: latest.clone(),
                update_command: format!("mvn dependency:get -Dartifact={}:{}:LATEST", g, a),
                url: format!("https://search.maven.org/artifact/{}/{}", g, a),
            });
        }
    }
    deps
}

// Gradle helpers
fn gradle_build_path(dir: &Path) -> PathBuf {
    if dir.join("build.gradle.kts").exists() {
        dir.join("build.gradle.kts")
    } else {
        dir.join("build.gradle")
    }
}

fn parse_gradle_deps(data: &str) -> Vec<(String, String, String)> {
    let mut deps = Vec::new();
    let mut in_block = false;
    for line in data.lines() {
        let l = line.trim();
        if l.starts_with("dependencies") {
            in_block = true;
            continue;
        }
        if in_block && l.starts_with('}') {
            in_block = false;
            continue;
        }
        if in_block {
            let configs = [
                "testImplementation",
                "testCompile",
                "testRuntimeOnly",
                "testCompileOnly",
            ];
            for cfg in configs {
                if l.starts_with(cfg) {
                    if let Some(start) = l.find("'").or_else(|| l.find("\"")) {
                        let quote = l.chars().nth(start).unwrap();
                        if let Some(end) = l[start + 1..].find(quote) {
                            let dep = &l[start + 1..start + 1 + end];
                            let mut parts = dep.split(':');
                            let g = parts.next().unwrap_or("").to_string();
                            let a = parts.next().unwrap_or("").to_string();
                            let v = parts.next().unwrap_or("").to_string();
                            deps.push((g, a, v));
                        }
                    }
                }
            }
        }
    }
    deps
}

fn list_gradle(dir: &Path) {
    let path = gradle_build_path(dir);
    if let Ok(data) = fs::read_to_string(&path) {
        let deps = parse_gradle_deps(&data);
        if deps.is_empty() {
            println!("Nenhuma dependência encontrada.");
        } else {
            for (g, a, v) in deps {
                println!("- {}:{} = {}", g, a, v);
            }
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn add_gradle(_dir: &Path, _name: String, _version: Option<String>) {
    println!("Operação não suportada para Gradle.");
}

fn update_gradle(_dir: &Path, _name: Option<String>) {
    println!("Operação não suportada para Gradle.");
}

fn delete_gradle(_dir: &Path, _name: String) {
    println!("Operação não suportada para Gradle.");
}

fn get_gradle_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = gradle_build_path(dir);
    let mut deps = Vec::new();
    if let Ok(data) = fs::read_to_string(&path) {
        for (g, a, v) in parse_gradle_deps(&data) {
            let latest = fetch_latest_maven(&g, &a);
            let name = format!("{}:{}", g, a);
            deps.push(DependencyInfo {
                name: name.clone(),
                current_version: v.clone(),
                latest_version: latest.clone(),
                update_command: "./gradlew --refresh-dependencies".into(),
                url: format!("https://search.maven.org/artifact/{}/{}", g, a),
            });
        }
    }
    deps
}

// PHP helpers
fn composer_json_path(dir: &Path) -> PathBuf {
    dir.join("composer.json")
}

fn load_composer_json(path: &Path) -> Value {
    let data = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Value::Object(Default::default()))
}

fn save_composer_json(path: &Path, v: &Value) {
    if let Ok(data) = serde_json::to_string_pretty(v) {
        let _ = fs::write(path, data);
    }
}

fn list_php(dir: &Path) {
    let path = composer_json_path(dir);
    let v = load_composer_json(&path);
    if let Some(obj) = v.get("require-dev").and_then(|d| d.as_object()) {
        for (k, v) in obj {
            if let Some(ver) = v.as_str() {
                println!("- {} = {}", k, ver);
            }
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn add_php(dir: &Path, name: String, version: Option<String>) {
    let path = composer_json_path(dir);
    let mut v = load_composer_json(&path);
    let obj = v
        .as_object_mut()
        .unwrap()
        .entry("require-dev")
        .or_insert_with(|| Value::Object(Default::default()));
    if let Some(map) = obj.as_object_mut() {
        map.insert(name.clone(), Value::String(version.unwrap_or("*".into())));
        save_composer_json(&path, &v);
        println!("Dependência '{name}' adicionada.");
    }
}

fn fetch_latest_packagist(name: &str) -> Option<String> {
    let url = format!("https://repo.packagist.org/p2/{}.json", name);
    let v = reqwest::blocking::get(url).ok()?.json::<Value>().ok()?;
    v.get("packages")?
        .as_object()?
        .get(name)?
        .get(0)?
        .get("version")?
        .as_str()
        .map(|s| s.trim_start_matches('v').to_string())
}

fn update_php(dir: &Path, name: Option<String>) {
    let path = composer_json_path(dir);
    let mut v = load_composer_json(&path);
    if let Some(map) = v.get_mut("require-dev").and_then(|d| d.as_object_mut()) {
        if let Some(n) = name {
            if let Some(latest) = fetch_latest_packagist(&n) {
                map.insert(n.clone(), Value::String(latest));
                println!("Dependência '{n}' atualizada.");
            }
        } else {
            for (k, val) in map.iter_mut() {
                if let Some(latest) = fetch_latest_packagist(k) {
                    *val = Value::String(latest);
                }
            }
            println!("Todas as dependências atualizadas.");
        }
        save_composer_json(&path, &v);
    }
}

fn delete_php(dir: &Path, name: String) {
    let path = composer_json_path(dir);
    let mut v = load_composer_json(&path);
    if let Some(map) = v
        .as_object_mut()
        .and_then(|o| o.get_mut("require-dev"))
        .and_then(|d| d.as_object_mut())
    {
        map.remove(&name);
        println!("Dependência '{name}' removida.");
    }
    save_composer_json(&path, &v);
}

fn get_php_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = composer_json_path(dir);
    let mut deps = Vec::new();
    let v = load_composer_json(&path);
    if let Some(map) = v.get("require-dev").and_then(|d| d.as_object()) {
        for (k, val) in map {
            if let Some(ver) = val.as_str() {
                let latest = fetch_latest_packagist(k);
                deps.push(DependencyInfo {
                    name: k.clone(),
                    current_version: ver.to_string(),
                    latest_version: latest.clone(),
                    update_command: format!("composer update {}", k),
                    url: format!("https://packagist.org/packages/{}", k),
                });
            }
        }
    }
    deps
}

// Ruby helpers
fn gemfile_path(dir: &Path) -> PathBuf {
    dir.join("Gemfile")
}

fn parse_gemfile(data: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let mut in_group = false;
    for line in data.lines() {
        let l = line.trim();
        if l.starts_with("group") {
            in_group = l.contains(":development") || l.contains(":test");
            continue;
        }
        if l == "end" {
            in_group = false;
            continue;
        }
        if in_group && l.starts_with("gem ") {
            let mut parts = l.splitn(2, ' ');
            parts.next();
            if let Some(rest) = parts.next() {
                let rest = rest.trim();
                let mut pieces = rest.split(',');
                let name = pieces
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_matches(|c| "\"'".contains(c))
                    .to_string();
                let version = pieces
                    .next()
                    .map(|v| v.trim().trim_matches(|c| "\"'".contains(c)).to_string())
                    .unwrap_or_else(|| "*".into());
                map.insert(name, version);
            }
        }
    }
    map
}

fn list_ruby(dir: &Path) {
    let path = gemfile_path(dir);
    if let Ok(data) = fs::read_to_string(&path) {
        let map = parse_gemfile(&data);
        if map.is_empty() {
            println!("Nenhuma dependência encontrada.");
        } else {
            for (k, v) in map {
                println!("- {} = {}", k, v);
            }
        }
    } else {
        println!("Nenhuma dependência encontrada.");
    }
}

fn add_ruby(_dir: &Path, _name: String, _version: Option<String>) {
    println!("Operação não suportada para Ruby.");
}

fn update_ruby(_dir: &Path, _name: Option<String>) {
    println!("Operação não suportada para Ruby.");
}

fn delete_ruby(_dir: &Path, _name: String) {
    println!("Operação não suportada para Ruby.");
}

fn fetch_latest_ruby(name: &str) -> Option<String> {
    let url = format!("https://rubygems.org/api/v1/gems/{}.json", name);
    reqwest::blocking::get(url)
        .ok()?
        .json::<Value>()
        .ok()?
        .get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn get_ruby_dependencies(dir: &Path) -> Vec<DependencyInfo> {
    let path = gemfile_path(dir);
    let mut deps = Vec::new();
    if let Ok(data) = fs::read_to_string(&path) {
        for (k, v) in parse_gemfile(&data) {
            let latest = fetch_latest_ruby(&k);
            deps.push(DependencyInfo {
                name: k.clone(),
                current_version: v.clone(),
                latest_version: latest.clone(),
                update_command: format!("bundle update {}", k),
                url: format!("https://rubygems.org/gems/{}", k),
            });
        }
    }
    deps
}
