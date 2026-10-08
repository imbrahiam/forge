use forge_runtime::{Runtime, Shell};
use std::{ffi::OsString, fs};

#[test]
fn bash_launch_preserves_paths_with_spaces_and_selects_ucrt64() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("MSYS2 runtime");
    let cwd = temp.path().join("my project");
    fs::create_dir_all(root.join("usr/bin")).unwrap();
    fs::create_dir_all(&cwd).unwrap();
    fs::write(root.join("usr/bin/bash.exe"), b"fixture").unwrap();
    fs::write(root.join("usr/bin/msys-2.0.dll"), b"fixture").unwrap();
    fs::create_dir_all(root.join("etc")).unwrap();
    fs::write(root.join("etc/profile"), b"fixture").unwrap();
    let plan = Runtime::new(&root).plan_shell(Shell::Bash, &cwd).unwrap();
    assert_eq!(plan.executable, root.join("usr/bin/bash.exe"));
    assert_eq!(plan.args, [OsString::from("--login"), OsString::from("-i")]);
    assert_eq!(plan.cwd, cwd);
    assert_eq!(
        plan.env.get(&OsString::from("MSYSTEM")),
        Some(&OsString::from("UCRT64"))
    );
    assert_eq!(
        plan.env.get(&OsString::from("CHERE_INVOKING")),
        Some(&OsString::from("1"))
    );
    assert_eq!(
        plan.env.get(&OsString::from("MSYS2_PATH_TYPE")),
        Some(&OsString::from("inherit"))
    );
}

#[test]
fn missing_bash_is_reported_before_a_process_can_be_launched() {
    let temp = tempfile::tempdir().unwrap();
    let error = Runtime::new(temp.path())
        .plan_shell(Shell::Bash, temp.path())
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    assert!(error.to_string().contains("bash.exe"), "{error}");
}

#[test]
fn a_shell_without_its_posix_runtime_is_not_a_valid_launch() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("usr/bin")).unwrap();
    fs::write(temp.path().join("usr/bin/bash.exe"), b"fixture").unwrap();
    let error = Runtime::new(temp.path())
        .plan_shell(Shell::Bash, temp.path())
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    assert!(error.to_string().contains("msys-2.0.dll"), "{error}");
}

#[test]
fn zsh_is_selected_explicitly_without_falling_back_to_bash() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("usr/bin")).unwrap();
    fs::write(temp.path().join("usr/bin/zsh.exe"), b"fixture").unwrap();
    fs::write(temp.path().join("usr/bin/msys-2.0.dll"), b"fixture").unwrap();
    fs::create_dir_all(temp.path().join("etc")).unwrap();
    fs::write(temp.path().join("etc/profile"), b"fixture").unwrap();
    fs::write(temp.path().join("etc/zprofile"), b"fixture").unwrap();
    let plan = Runtime::new(temp.path())
        .plan_shell(Shell::Zsh, temp.path())
        .unwrap();
    assert_eq!(plan.executable, temp.path().join("usr/bin/zsh.exe"));
    assert_eq!(plan.args, [OsString::from("--login"), OsString::from("-i")]);
}

#[test]
fn login_launch_requires_the_profile_that_initializes_ucrt64() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("usr/bin")).unwrap();
    for file in ["bash.exe", "msys-2.0.dll"] {
        fs::write(temp.path().join("usr/bin").join(file), b"fixture").unwrap();
    }
    let error = Runtime::new(temp.path())
        .plan_shell(Shell::Bash, temp.path())
        .unwrap_err();
    assert!(error.to_string().contains("profile"), "{error}");
}

#[test]
fn a_non_directory_working_path_is_rejected() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("usr/bin")).unwrap();
    fs::create_dir_all(temp.path().join("etc")).unwrap();
    for file in [
        "usr/bin/bash.exe",
        "usr/bin/msys-2.0.dll",
        "etc/profile",
        "file.txt",
    ] {
        fs::write(temp.path().join(file), b"fixture").unwrap();
    }
    let error = Runtime::new(temp.path())
        .plan_shell(Shell::Bash, temp.path().join("file.txt"))
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("working directory"), "{error}");
}

#[test]
fn zsh_requires_the_profile_bridge_to_msys2_initialization() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("usr/bin")).unwrap();
    fs::create_dir_all(temp.path().join("etc")).unwrap();
    for file in ["usr/bin/zsh.exe", "usr/bin/msys-2.0.dll", "etc/profile"] {
        fs::write(temp.path().join(file), b"fixture").unwrap();
    }
    let error = Runtime::new(temp.path())
        .plan_shell(Shell::Zsh, temp.path())
        .unwrap_err();
    assert!(error.to_string().contains("zprofile"), "{error}");
}

#[test]
fn relative_runtime_paths_are_resolved_before_the_child_changes_directory() {
    let current = std::env::current_dir().unwrap();
    let temp = tempfile::tempdir_in(&current).unwrap();
    std::fs::create_dir_all(temp.path().join("usr/bin")).unwrap();
    std::fs::create_dir_all(temp.path().join("etc")).unwrap();
    for file in ["usr/bin/bash.exe", "usr/bin/msys-2.0.dll", "etc/profile"] {
        fs::write(temp.path().join(file), b"fixture").unwrap();
    }
    let relative = temp.path().strip_prefix(&current).unwrap();
    let plan = Runtime::new(relative).plan_shell(Shell::Bash, ".").unwrap();
    assert_eq!(plan.executable, temp.path().join("usr/bin/bash.exe"));
    assert_eq!(plan.cwd, current);
}
