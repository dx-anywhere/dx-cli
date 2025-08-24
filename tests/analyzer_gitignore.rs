use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

#[test]
fn analyzer_adds_dx_to_gitignore_when_missing() {
    // Prepare an isolated temp project directory
    let tmp = env::temp_dir();
    let test_dir = tmp.join(format!("dx-cli-analyzer-gitignore-{}",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).expect("failed to create test_dir");

    // Add a marker file to look like a project root
    fs::write(test_dir.join("Cargo.toml"), "[package]\nname='demo'\nversion='0.1.0'\n").unwrap();

    // Ensure there is no .gitignore initially
    assert!(!test_dir.join(".gitignore").exists());

    // Run analyzer without saving report
    let exe = env!("CARGO_BIN_EXE_dx");
    let status = Command::new(exe)
        .arg("analyzer")
        .arg("--no-save")
        .arg(test_dir.to_string_lossy().to_string())
        .status()
        .expect("failed to run analyzer");
    assert!(status.success());

    // .gitignore should be created with .dx entry
    let gi_path = test_dir.join(".gitignore");
    let content = fs::read_to_string(&gi_path).expect(".gitignore should exist");
    assert!(content.lines().any(|l| l.trim() == ".dx"), ".gitignore missing .dx entry; content was: {}", content);

    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn analyzer_does_not_duplicate_dx_entry() {
    // Prepare an isolated temp project directory
    let tmp = env::temp_dir();
    let test_dir = tmp.join(format!("dx-cli-analyzer-gitignore-dupe-{}",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()));
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).expect("failed to create test_dir");

    // Marker for project root
    fs::write(test_dir.join("Cargo.toml"), "[package]\nname='demo'\nversion='0.1.0'\n").unwrap();

    // Pre-create .gitignore without trailing newline to check newline handling
    let mut f = fs::File::create(test_dir.join(".gitignore")).unwrap();
    write!(f, "target\n.DS_Store").unwrap();

    // Run analyzer without saving
    let exe = env!("CARGO_BIN_EXE_dx");
    let status = Command::new(exe)
        .arg("analyzer")
        .arg("--no-save")
        .arg(test_dir.to_string_lossy().to_string())
        .status()
        .expect("failed to run analyzer");
    assert!(status.success());

    // Verify single .dx line exists
    let content = fs::read_to_string(test_dir.join(".gitignore")).unwrap();
    let dx_count = content.lines().filter(|l| l.trim() == ".dx").count();
    assert_eq!(dx_count, 1, "expected exactly one .dx entry, got {} in: {}", dx_count, content);

    let _ = fs::remove_dir_all(&test_dir);
}
