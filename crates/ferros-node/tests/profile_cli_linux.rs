use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferros_profile::ProfileDocument;

#[test]
fn profile_init_then_show_works_via_real_ferros_binary() {
    let profile_path = unique_profile_path();
    let expected_init_line = format!("initialized profile at {}", profile_path.display());

    let init_output = Command::new(env!("CARGO_BIN_EXE_ferros"))
        .args(["profile", "init"])
        .arg(&profile_path)
        .output()
        .expect("ferros profile init should launch");

    assert!(
        init_output.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&init_output.stderr)
    );

    let init_stdout =
        String::from_utf8(init_output.stdout).expect("init stdout should be valid UTF-8");
    assert!(
        init_stdout.lines().any(|line| line == expected_init_line),
        "unexpected init stdout: {init_stdout}"
    );

    let show_output = Command::new(env!("CARGO_BIN_EXE_ferros"))
        .args(["profile", "show"])
        .arg(&profile_path)
        .output()
        .expect("ferros profile show should launch");

    assert!(
        show_output.status.success(),
        "show failed: {}",
        String::from_utf8_lossy(&show_output.stderr)
    );

    let show_stdout =
        String::from_utf8(show_output.stdout).expect("show stdout should be valid UTF-8");
    let profile =
        ProfileDocument::from_json_str(&show_stdout).expect("show stdout should be valid profile JSON");

    assert_eq!(profile.identity.name, "Fresh Start");
    assert!(profile.has_genesis_seal());

    cleanup_profile_path(&profile_path);
}

fn unique_profile_path() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();

    std::env::temp_dir()
        .join("ferros-profile-cli-linux-tests")
        .join(format!("profile-{nonce}.json"))
}

fn cleanup_profile_path(path: &Path) {
    let _ = fs::remove_file(path);

    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir_all(parent);
    }
}