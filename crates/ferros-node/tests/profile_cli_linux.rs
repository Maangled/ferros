use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferros_profile::{FileSystemProfileStore, LocalProfileStore, ProfileDocument};
use serde_json::Value;

#[test]
fn profile_cli_lifecycle_works_via_real_ferros_binary() {
    let profile_path = unique_profile_path("source");
    let bundle_path = unique_bundle_path();
    let imported_profile_path = unique_profile_path("imported");
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

    let grant_output = Command::new(env!("CARGO_BIN_EXE_ferros"))
        .args(["profile", "grant", "agent.echo"])
        .arg(&profile_path)
        .output()
        .expect("ferros profile grant should launch");

    assert!(
        grant_output.status.success(),
        "grant failed: {}",
        String::from_utf8_lossy(&grant_output.stderr)
    );

    let grant_stdout =
        String::from_utf8(grant_output.stdout).expect("grant stdout should be valid UTF-8");
    assert!(
        grant_stdout
            .lines()
            .any(|line| line.starts_with("granted agent.echo to ")),
        "unexpected grant stdout: {grant_stdout}"
    );

    let export_output = Command::new(env!("CARGO_BIN_EXE_ferros"))
        .args(["profile", "export"])
        .arg(&bundle_path)
        .arg(&profile_path)
        .output()
        .expect("ferros profile export should launch");

    assert!(
        export_output.status.success(),
        "export failed: {}",
        String::from_utf8_lossy(&export_output.stderr)
    );

    let import_output = Command::new(env!("CARGO_BIN_EXE_ferros"))
        .args(["profile", "import"])
        .arg(&bundle_path)
        .arg(&imported_profile_path)
        .output()
        .expect("ferros profile import should launch");

    assert!(
        import_output.status.success(),
        "import failed: {}",
        String::from_utf8_lossy(&import_output.stderr)
    );

    let store = FileSystemProfileStore;
    let source_state = store
        .load_local_profile(&profile_path)
        .expect("source local profile should load");
    let imported_state = store
        .load_local_profile(&imported_profile_path)
        .expect("imported local profile should load");

    assert_eq!(imported_state.profile, source_state.profile);
    assert_eq!(
        imported_state.key_pair.public_key_hex(),
        source_state.key_pair.public_key_hex()
    );
    assert_eq!(
        imported_state.key_pair.secret_key_hex(),
        source_state.key_pair.secret_key_hex()
    );
    assert_eq!(imported_state.signed_grants, source_state.signed_grants);

    let revoke_output = Command::new(env!("CARGO_BIN_EXE_ferros"))
        .args(["profile", "revoke", "agent.echo"])
        .arg(&imported_profile_path)
        .output()
        .expect("ferros profile revoke should launch");

    assert!(
        revoke_output.status.success(),
        "revoke failed: {}",
        String::from_utf8_lossy(&revoke_output.stderr)
    );

    let revoke_stdout =
        String::from_utf8(revoke_output.stdout).expect("revoke stdout should be valid UTF-8");
    assert!(
        revoke_stdout
            .lines()
            .any(|line| line.starts_with("revoked agent.echo for ")),
        "unexpected revoke stdout: {revoke_stdout}"
    );

    let show_output = Command::new(env!("CARGO_BIN_EXE_ferros"))
        .args(["profile", "show"])
        .arg(&imported_profile_path)
        .output()
        .expect("ferros profile show should launch");

    assert!(
        show_output.status.success(),
        "show failed: {}",
        String::from_utf8_lossy(&show_output.stderr)
    );

    let show_stdout =
        String::from_utf8(show_output.stdout).expect("show stdout should be valid UTF-8");
    let show_json: Value =
        serde_json::from_str(&show_stdout).expect("show stdout should be valid JSON");
    let profile = ProfileDocument::from_json_str(&show_stdout)
        .expect("show stdout should be valid profile JSON");

    assert_unsigned_profile_boundary(&show_json);
    assert_eq!(profile.identity.name, "Fresh Start");
    assert!(profile.has_genesis_seal());

    let imported_state_after_revoke = store
        .load_local_profile(&imported_profile_path)
        .expect("imported local profile should still load after revoke");
    assert_eq!(imported_state_after_revoke.signed_grants.len(), 1);

    let revoked_grant_json = serde_json::to_value(&imported_state_after_revoke.signed_grants[0])
        .expect("revoked signed grant should serialize");
    imported_state_after_revoke.signed_grants[0]
        .verify()
        .expect("revoked imported signed grant should still verify");
    assert_signed_grant_matches_frozen_boundary(&revoked_grant_json);
    assert!(imported_state_after_revoke.signed_grants[0]
        .grant
        .is_revoked());

    cleanup_profile_path(&profile_path);
    cleanup_bundle_path(&bundle_path);
    cleanup_profile_path(&imported_profile_path);
}

fn unique_profile_path(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();

    std::env::temp_dir()
        .join("ferros-profile-cli-linux-tests")
        .join(format!("profile-{label}-{nonce}.json"))
}

fn unique_bundle_path() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();

    std::env::temp_dir()
        .join("ferros-profile-cli-linux-tests")
        .join(format!("bundle-{nonce}.json"))
}

fn assert_unsigned_profile_boundary(value: &Value) {
    let object = value
        .as_object()
        .expect("show output should be a JSON object");

    assert!(object.contains_key("meta"));
    assert!(object.contains_key("identity"));
    assert!(!object.contains_key("profile_id"));
    assert!(!object.contains_key("profile"));
    assert!(!object.contains_key("signer_public_key"));
    assert!(!object.contains_key("signature"));
}

fn assert_signed_grant_matches_frozen_boundary(value: &Value) {
    let object = value
        .as_object()
        .expect("signed grant should serialize as a JSON object");
    let mut fields = object.keys().cloned().collect::<Vec<_>>();
    fields.sort();

    assert_eq!(
        fields,
        vec![
            "capability".to_string(),
            "profile_id".to_string(),
            "revocation_reason".to_string(),
            "revoked_at".to_string(),
            "signature".to_string(),
            "signer_public_key".to_string(),
        ]
    );
    assert!(object.get("profile_id").is_some_and(Value::is_string));
    assert!(object.get("capability").is_some_and(Value::is_string));
    assert!(object.get("revoked_at").is_some_and(Value::is_string));
    assert!(object
        .get("revocation_reason")
        .is_some_and(Value::is_string));
    assert!(object
        .get("signer_public_key")
        .is_some_and(Value::is_string));
    assert!(object.get("signature").is_some_and(Value::is_string));
}

fn cleanup_profile_path(path: &Path) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(path.with_extension("key.json"));
    let _ = fs::remove_file(path.with_extension("grants.json"));

    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir_all(parent);
    }
}

fn cleanup_bundle_path(path: &Path) {
    let _ = fs::remove_file(path);

    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir_all(parent);
    }
}
