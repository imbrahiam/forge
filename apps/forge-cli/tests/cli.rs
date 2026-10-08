use std::process::Command;

#[test]
fn help_describes_available_commands_without_requiring_a_runtime() {
    let output = Command::new(env!("CARGO_BIN_EXE_forge"))
        .arg("--help")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout.contains("doctor") && stdout.contains("plan"),
        "{stdout}"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn plan_prints_a_curated_target_and_states_that_nothing_was_installed() {
    let output = Command::new(env!("CARGO_BIN_EXE_forge"))
        .args(["plan", "ripgrep"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout.contains("ucrt64/mingw-w64-ucrt-x86_64-ripgrep"),
        "{stdout}"
    );
    assert!(stdout.contains("No packages installed"), "{stdout}");
}

#[test]
fn doctor_reports_file_checks_without_claiming_to_have_executed_windows_binaries() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("runtime with spaces");
    std::fs::create_dir_all(root.join("usr/bin")).unwrap();
    std::fs::create_dir_all(root.join("etc")).unwrap();
    for file in ["usr/bin/bash.exe", "usr/bin/msys-2.0.dll", "etc/profile"] {
        std::fs::write(root.join(file), b"fixture, not a real Windows binary").unwrap();
    }
    let output = Command::new(env!("CARGO_BIN_EXE_forge"))
        .args(["doctor", "--runtime-root"])
        .arg(&root)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout.contains("bash.exe") && stdout.contains("UCRT64"),
        "{stdout}"
    );
    assert!(stdout.contains("not executed"), "{stdout}");
}

#[test]
fn doctor_can_inspect_zsh_without_requiring_bash() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("usr/bin")).unwrap();
    std::fs::create_dir_all(temp.path().join("etc")).unwrap();
    for file in [
        "usr/bin/zsh.exe",
        "usr/bin/msys-2.0.dll",
        "etc/profile",
        "etc/zprofile",
    ] {
        std::fs::write(temp.path().join(file), b"fixture").unwrap();
    }
    let output = Command::new(env!("CARGO_BIN_EXE_forge"))
        .args(["doctor", "--shell", "zsh", "--runtime-root"])
        .arg(temp.path())
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("zsh.exe"),
        "{output:?}"
    );
}
