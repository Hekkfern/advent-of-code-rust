#[derive(serde::Deserialize)]
struct CargoMetadata {
    workspace_root: std::path::PathBuf,
}

pub fn get_workspace_root() -> std::path::PathBuf {
    let output = std::process::Command::new("cargo")
        .args(["metadata"])
        .output()
        .unwrap();
    let metadata: CargoMetadata = serde_json::from_slice(&output.stdout).unwrap();
    metadata.workspace_root
}
