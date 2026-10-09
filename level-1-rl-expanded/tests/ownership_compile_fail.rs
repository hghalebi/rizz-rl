//! Verify the deliberate diagnostic fails specifically for reuse after a move.
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn rejects_reuse_after_move() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let output_dir =
        std::env::temp_dir().join(format!("rizz-rl-ownership-{}-{unique}", std::process::id()));
    std::fs::create_dir(&output_dir).expect("create isolated compiler output directory");
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ownership_move.rs");
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = Command::new(rustc)
        .args([
            "--edition=2021",
            "--error-format=json",
            "--emit=metadata",
            "--out-dir",
        ])
        .arg(&output_dir)
        .arg(fixture)
        .output()
        .expect("run rustc for the ownership diagnostic");
    std::fs::remove_dir_all(&output_dir).expect("remove compiler output directory");
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "the move-error fixture unexpectedly compiled"
    );
    assert!(
        diagnostic.contains("\"code\":\"E0382\""),
        "expected E0382 for reuse after move, got {diagnostic}"
    );
    println!("Observed: rustc rejects reuse of label with E0382 after consume takes ownership.");
    println!("Explain which expression moved label, and how a borrowed parameter would differ.");
}
