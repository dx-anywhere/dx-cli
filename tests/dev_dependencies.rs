use std::process::Command;

fn run_list(dir: &str, needle: &str) {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-dependencies", "list"])
        .current_dir(dir)
        .output()
        .expect("failed to run dx dev-dependencies list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(needle), "output did not contain {}", needle);
}

#[test]
fn list_node() {
    run_list("test-projects/nodejs", "express");
}

#[test]
fn list_python() {
    run_list("test-projects/python", "Flask");
}

#[test]
fn list_php() {
    run_list("test-projects/php", "monolog/monolog");
}

#[test]
fn list_ruby() {
    run_list("test-projects/ruby", "rails");
}
