use crate::{error::SlitherError, types::SlitherResult, utils::normalize_slither_path};
use std::process::Stdio;
use tokio::{io::AsyncReadExt, process::Command};
use tower_lsp::lsp_types::Diagnostic;

pub async fn parse_slither_out(uri: &str) -> Result<Vec<Diagnostic>, SlitherError> {
    let mut results: Vec<Diagnostic> = Vec::new();

    eprintln!("SLITHER STARTING");
    let mut output = exec_slither(uri)?;
    let out = match output.stdout.take() {
        Some(out) => out,
        None => {
            return Err(SlitherError::Unknown(
                "Failed to get slither output pipe".to_string(),
            ))
        }
    };

    let mut buffer = tokio::io::BufReader::new(out);
    let mut dst = String::new();

    output.wait().await?;
    eprintln!("SLITHER FINISHED");

    buffer.read_to_string(&mut dst).await?;
    let json: Result<SlitherResult, serde_json::Error> = serde_json::from_str(&dst);

    match json {
        Ok(json) => {
            for detector in json.results.detectors {
                results.append(&mut crate::types::diag_from_json(detector.clone()));
            }
        }
        Err(e) => {
            eprintln!("Error parsing slither output: {}", e);
            return Err(SlitherError::ParsingFailed(e));
        }
    }

    Ok(results)
}

fn exec_slither(uri: &str) -> Result<tokio::process::Child, std::io::Error> {
    Command::new("slither")
        .arg(normalize_slither_path(uri))
        .arg("--exclude")
        .arg("naming-convention")
        .arg("--json")
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
}
