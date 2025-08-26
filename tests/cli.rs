use std::process::Command;
use std::fs;

#[test]
fn help_lists_subcommands() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .arg("--help")
        .output()
        .expect("failed to run dx --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Basic identity and description
    assert!(stdout.contains("dx"));
    assert!(stdout.contains("DX em qualquer stack"));

    // Subcommands (kebab-case by default via clap derive)
    for sub in [
        "dev-services",
        "dev-test",
        "dev-config",
        "dev-dependencies",
        "portal",
        "tests",
        "config",
        "docs",
        "governance",
    ] {
        assert!(stdout.contains(sub), "missing subcommand in help: {}", sub);
    }
}

#[test]
fn dev_config_lists_configs() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .args(["dev-config", "list"])
        .current_dir("test-projects/nodejs")
        .output()
        .expect("failed to run dx dev-config list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Stack detectada"));
}

#[test]
fn dev_config_add_update_delete() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"tmp\"\nversion=\"0.1.0\"").unwrap();

    let status = Command::new(exe)
        .args(["dev-config", "add", "foo", "bar"])
        .current_dir(tmp.path())
        .status()
        .expect("failed to run add");
    assert!(status.success());

    let path = tmp.path().join(".dx").join("config.json");
    let contents = fs::read_to_string(&path).expect("read config");
    assert!(contents.contains("\"foo\": \"bar\""));

    let status = Command::new(exe)
        .args(["dev-config", "update", "foo", "baz"])
        .current_dir(tmp.path())
        .status()
        .expect("failed to run update");
    assert!(status.success());
    let contents = fs::read_to_string(&path).expect("read config");
    assert!(contents.contains("\"foo\": \"baz\""));

    let status = Command::new(exe)
        .args(["dev-config", "delete", "foo"])
        .current_dir(tmp.path())
        .status()
        .expect("failed to run delete");
    assert!(status.success());
    let contents = fs::read_to_string(&path).expect("read config");
    assert!(!contents.contains("foo"));
}
