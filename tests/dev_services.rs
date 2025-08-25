// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 The dx-cli Contributors
use std::env;
use std::fs;
use std::process::Command;

// Test that the dev-services command correctly displays help information
#[test]
fn dev_services_command_shows_help() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .arg("dev-services")
        .arg("--help")
        .output()
        .expect("failed to run dx-cli dev-services --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that the help output contains information about the no-save flag
    assert!(
        stdout.contains("--no-save"),
        "Help output doesn't mention --no-save option"
    );
    assert!(
        stdout.contains("Não salva o manifesto"),
        "Help output doesn't explain no-save option"
    );
}

// Test that the dev-services command works without arguments
#[test]
fn dev_services_command_runs_without_args() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .arg("dev-services")
        .output()
        .expect("failed to run dx-cli dev-services");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for presence of key phrases in the output
    assert!(
        stdout.contains("Manifesto Dev Services"),
        "Missing expected output header"
    );

    // Ensure it mentions docker-compose
    assert!(
        stdout.contains("docker-compose.yml") || stdout.contains("Nenhuma dependência detectada"),
        "Output missing expected docker-compose.yml reference or dependency message"
    );
}

// Test that the dev-services command saves docker-compose.yml by default
#[test]
fn dev_services_default_save_works() {
    // Get the path to the temporary directory for testing
    let temp_dir = env::temp_dir();
    let test_dir = temp_dir.join("dx-cli-test");

    // Create a clean test directory
    let _ = fs::remove_dir_all(&test_dir); // Ignore if it doesn't exist
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");

    // Create a dummy Cargo.toml with a postgres dependency for testing in the target test directory
    fs::write(
        test_dir.join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
postgres = "0.19"
"#,
    )
    .expect("Failed to create test Cargo.toml");

    // Run the command pointing to the test directory (default behavior saves by default)
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .arg("dev-services")
        .arg(test_dir.to_string_lossy().to_string())
        .output()
        .expect("failed to run dx-cli dev-services");

    assert!(output.status.success());

    // Check if docker-compose.yml was created under .dx directory of the test dir
    let docker_compose_path = test_dir.join(".dx").join("docker-compose.yml");
    let file_exists = docker_compose_path.exists();

    if file_exists {
        // Verify file content contains postgres service
        let content =
            fs::read_to_string(docker_compose_path).expect("Failed to read docker-compose.yml");
        assert!(
            content.contains("postgres"),
            "docker-compose.yml doesn't contain postgres service"
        );
    } else {
        // If no file was created, output should explain why
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Não há dependências"),
            "No docker-compose.yml created and no explanation given"
        );
    }

    // Clean up test directory (optional, can be left for inspection)
    let _ = fs::remove_dir_all(&test_dir);
}

// Test that using --no-save flag prevents saving the docker-compose.yml file
#[test]
fn dev_services_no_save_flag_works() {
    // Get the path to the temporary directory for testing
    let temp_dir = env::temp_dir();
    // Use a unique directory for this test with a timestamp to ensure isolation
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_dir = temp_dir.join(format!("dx-cli-test-no-save-{}", timestamp));

    // Create a clean test directory
    let _ = fs::remove_dir_all(&test_dir); // Ignore if it doesn't exist
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");

    // Create a dummy Cargo.toml with a postgres dependency for testing in test_dir
    fs::write(
        test_dir.join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
postgres = "0.19"
"#,
    )
    .expect("Failed to create test Cargo.toml");

    // Run the command with --no-save flag, pointing to the test_dir
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .arg("dev-services")
        .arg("--no-save")
        .arg(test_dir.to_string_lossy().to_string())
        .output()
        .expect("failed to run dx-cli dev-services --no-save");

    assert!(output.status.success());

    // Add a small sleep to ensure file operations are complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Print output for debugging
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("DEBUG OUTPUT START =====================");
    println!("Command output: {}", stdout);
    println!("Current directory: {:?}", env::current_dir().unwrap());
    println!("Test directory: {:?}", test_dir);

    // Check that docker-compose.yml was NOT created under .dx in the test dir
    let docker_compose_path = test_dir.join(".dx").join("docker-compose.yml");
    let file_exists = docker_compose_path.exists();
    println!("File exists: {}", file_exists);
    println!(
        "File path: {:?}",
        docker_compose_path
            .canonicalize()
            .unwrap_or_else(|_| docker_compose_path.to_path_buf())
    );

    // List files in current directory
    println!("Files in current directory:");
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("  {:?}", entry.path());
            }
        }
    }
    println!("DEBUG OUTPUT END =======================");

    // File should not exist when using --no-save
    assert!(
        !file_exists,
        "docker-compose.yml was created despite using --no-save flag"
    );

    // Check that the output contains instructions about --no-save
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Para salvar este manifesto"),
        "Output should mention how to save the manifest when using --no-save"
    );

    // Clean up test directory
    let _ = fs::remove_dir_all(&test_dir);
}

// Test that the stop subcommand references the correct CLI name when compose file is missing
#[test]
fn dev_services_stop_shows_dx_cli_name() {
    // Create a temporary directory without a .dx/docker-compose.yml
    let temp_dir = env::temp_dir().join("dx-cli-stop-test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .arg("dev-services")
        .arg("stop")
        .arg(temp_dir.to_string_lossy().to_string())
        .output()
        .expect("failed to run dx-cli dev-services stop");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("dx dev-services"),
        "stderr should mention 'dx dev-services': {}",
        stderr
    );
    assert!(
        !stderr.contains("dxany"),
        "stderr contains outdated CLI name: {}",
        stderr
    );

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}
