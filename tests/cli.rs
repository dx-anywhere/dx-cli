use std::process::Command;

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
        "portal",
        "tests",
        "config",
        "docs",
        "governance",
    ] {
        assert!(stdout.contains(sub), "missing subcommand in help: {}", sub);
    }
}
