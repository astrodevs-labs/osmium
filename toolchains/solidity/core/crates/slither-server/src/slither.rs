use crate::error::SlitherError;
use crate::types::SlitherResult;
use std::process::Command;
use tower_lsp::lsp_types::Diagnostic;

pub fn is_slither_installed() -> bool {
    let output = Command::new("slither").arg("--version").output();

    match output {
        Err(_) => false,
        Ok(_) => true,
    }
}

pub fn is_solc_installed() -> bool {
    let output = Command::new("solc").arg("--version").output();

    match output {
        Err(_) => false,
        Ok(_) => true,
    }
}
#[cfg(target_family = "windows")]
fn normalize_slither_path(path: &str) -> String {
    let mut path = path.replace("%3A/", "://");
    path.remove(0);
    path.to_string()
}

#[cfg(not(target_family = "windows"))]
fn normalize_slither_path(path: &str) -> String {
    path.to_string()
}

pub fn exec_slither(filepath: &str) -> Result<Vec<Diagnostic>, SlitherError> {
    let mut results: Vec<Diagnostic> = Vec::new();
    let out = Command::new("slither")
        .arg(normalize_slither_path(filepath))
        .arg("--exclude")
        .arg("naming-convention")
        .arg("--json")
        .arg("-")
        .output()?;
    if out.status.code() == Some(1) {
        return Err(SlitherError::SlitherError);
    }
    if out.stdout.is_empty() {
        return Ok(results);
    }
    let json: SlitherResult =
        serde_json::from_str(&String::from_utf8_lossy(&out.stdout).replace("\\\"", "\""))?;
    for detector in json.results.detectors {
        results.append(&mut crate::types::diag_from_json(detector.clone()));
    }
    return Ok(results);
}
