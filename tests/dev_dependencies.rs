use std::fs;
use std::process::Command;
use tempfile;

#[test]
fn dev_dependencies_list_node() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/nodejs")
        .output()
        .expect("failed to run dx dev-dependencies list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("jest"));
}

#[test]
fn dev_dependencies_add_update_delete_node() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("package.json"), "{\n  \"devDependencies\": {}\n}\n").unwrap();

    let status = Command::new(exe)
        .args(["dev-dependencies", "add", "eslint", "1.0.0"])
        .current_dir(tmp.path())
        .status()
        .expect("run add");
    assert!(status.success());
    let pkg = fs::read_to_string(tmp.path().join("package.json")).unwrap();
    assert!(pkg.contains("eslint"));

    let status = Command::new(exe)
        .args(["dev-dependencies", "update", "eslint"])
        .current_dir(tmp.path())
        .status()
        .expect("run update");
    assert!(status.success());

    let status = Command::new(exe)
        .args(["dev-dependencies", "delete", "eslint"])
        .current_dir(tmp.path())
        .status()
        .expect("run delete");
    assert!(status.success());
    let pkg = fs::read_to_string(tmp.path().join("package.json")).unwrap();
    assert!(!pkg.contains("eslint"));
}

#[test]
fn dev_dependencies_list_python() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/python")
        .output()
        .expect("run list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pytest"));
}

#[test]
fn dev_dependencies_list_go() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/go")
        .output()
        .expect("run list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("github.com/gin-gonic/gin"));
}

#[test]
fn dev_dependencies_list_php() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/php")
        .output()
        .expect("run list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("phpunit/phpunit"));
}

#[test]
fn dev_dependencies_list_ruby() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/ruby")
        .output()
        .expect("run list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rspec-rails"));
}

#[test]
fn dev_dependencies_list_java_maven() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/java-maven")
        .output()
        .expect("run list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("spring-boot-starter-test"));
}

#[test]
fn dev_dependencies_list_java_gradle() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/java-gradle")
        .output()
        .expect("run list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("spring-boot-starter-test"));
}

#[test]
fn dev_dependencies_list_flink() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir("test-projects/flink")
        .output()
        .expect("run list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("flink-test-utils"));
}
