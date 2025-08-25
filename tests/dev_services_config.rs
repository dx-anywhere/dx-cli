// SPDX-License-Identifier: MIT OR Apache-2.0
use std::env;
use std::fs;
use std::process::Command;

#[test]
fn dev_services_config_set_list_remove() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let temp_dir = env::temp_dir().join("dx-cli-dev-services-config-test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("criar diretório de teste");

    // Set an env variable
    let status = Command::new(exe)
        .current_dir(&temp_dir)
        .arg("dev-services")
        .arg("config")
        .arg("set")
        .arg("env")
        .arg("API_KEY")
        .arg("123")
        .arg("--stack")
        .arg("spring-boot")
        .status()
        .expect("executar dev-services config set");
    assert!(status.success());

    // List and check presence
    let output = Command::new(exe)
        .current_dir(&temp_dir)
        .arg("dev-services")
        .arg("config")
        .arg("list")
        .arg("--stack")
        .arg("spring-boot")
        .output()
        .expect("executar dev-services config list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("API_KEY: 123"));

    // Remove the variable
    let status = Command::new(exe)
        .current_dir(&temp_dir)
        .arg("dev-services")
        .arg("config")
        .arg("remove")
        .arg("env")
        .arg("API_KEY")
        .arg("--stack")
        .arg("spring-boot")
        .status()
        .expect("executar dev-services config remove");
    assert!(status.success());

    // Ensure removal
    let output = Command::new(exe)
        .current_dir(&temp_dir)
        .arg("dev-services")
        .arg("config")
        .arg("list")
        .arg("--stack")
        .arg("spring-boot")
        .output()
        .expect("executar dev-services config list 2");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("API_KEY"));

    let _ = fs::remove_dir_all(&temp_dir);
}
