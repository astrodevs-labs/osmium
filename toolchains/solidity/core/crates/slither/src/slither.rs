use std::process::Command;
use tower_lsp::lsp_types::Diagnostic;
use crate::types::{SlitherResult};

pub fn is_slither_installed() -> bool {
    let output = Command::new("slither")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output = String::from_utf8_lossy(&output.stdout);

    output.contains("Slither version")
}

pub fn is_solc_installed() -> bool {
    let output = Command::new("solc")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output = String::from_utf8_lossy(&output.stdout);

    output.contains("Version")
}

pub fn exec_slither(filepath: &str) -> Vec<Diagnostic> {
    let mut results: Vec<Diagnostic> = Vec::new();
    let output = Command::new("slither")
        .arg(filepath.to_string())
        .arg("--json - ")
        .output()
        .expect("Failed to execute command");
    let json: SlitherResult = serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();

    for detector in json.results {
        println!("detector: {:?}", detector);
        for i in 0..detector.elements.len() {
            results.push(crate::types::from_json(detector.clone(), i));
        }
    }
    results
}