use crate::error::SlitherError;
use crate::types::SlitherResult;
use std::{error::Error, future::Future, process::Stdio};
use tokio::{io::{AsyncBufReadExt, AsyncReadExt}, process::{Command, Child}};
use std::process::Command as StdCommand;
use tower_lsp::lsp_types::Diagnostic;
use glob::glob;

pub fn is_slither_installed() -> bool {
    let output = StdCommand::new("slither").arg("--version").output();
    output.is_ok()
}

pub fn is_solc_installed() -> bool {
    let output = StdCommand::new("solc").arg("--version").output();
    output.is_ok()
}

#[cfg(target_family = "windows")]
pub fn normalize_slither_path(path: &str) -> String {
    let mut path = path.replace("%3A/", "://");
    path.remove(0);
    path.to_string()
}

#[cfg(not(target_family = "windows"))]
pub fn normalize_slither_path(path: &str) -> String {
    path.to_string()
}

struct SlitherProcess {
    child: std::process::Child,
}

impl SlitherProcess {
    pub fn create(uri: &str) -> Result<Self, std::io::Error> {
        Ok(Self {
            child:
            StdCommand::new("slither")
            .arg(normalize_slither_path(uri))
            .arg("--exclude")
            .arg("naming-convention")
            .arg("--json")
            .arg("-").spawn()?
        })
    }
}

impl Future for SlitherProcess {
    type Output = Result<String, SlitherError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.child.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    std::task::Poll::Ready(Ok("".to_string()))
                } else {
                    std::task::Poll::Ready(Err(SlitherError::Unknown))
                }
            }
            Ok(None) => std::task::Poll::Pending,
            Err(e) => std::task::Poll::Ready(Err(SlitherError::IoCommandError(e))),
        }
    }
} 

pub async fn parse_slither_out(uri: &str) -> Result<Vec<Diagnostic>, SlitherError> {
    let mut results: Vec<Diagnostic> = Vec::new();
    /*let proc = Box::pin(SlitherProcess::create(uri));

    match proc.await {
        Ok(out) => {
            let json: SlitherResult =
                serde_json::from_str(&out.replace("\\\"", "\""))?;
            for detector in json.results.detectors {
                results.append(&mut crate::types::diag_from_json(detector.clone()));
            }
            Ok(results)
        }
        Err(e) => Err(e),
    }*/
    /*
    eprintln!("SLITHER STARTING");
    let output = StdCommand::new("slither")
    .arg(normalize_slither_path(uri))
    .arg("--exclude")
    .arg("naming-convention")
    .arg("--json")
    .arg("-").output();
    eprintln!("SLITHER FINISHED");
    match output {
        Ok(output) => {
            let out_str = String::from_utf8_lossy(&output.stdout).to_string();
            if out_str.is_empty() {
                eprintln!("SLITHER EMPTY OUT: {}", String::from_utf8_lossy(&output.stderr).to_string());
                return Ok(results);
            }
            let json: SlitherResult =
                serde_json::from_str(&out_str)?;
            for detector in json.results.detectors {
                results.append(&mut crate::types::diag_from_json(detector.clone()));
            }
        }
        Err(e) => {
            eprintln!("SLITHER ERROR: {:?}", e);
        }
    }*/

    let mut output = Command::new("slither")
    .arg(normalize_slither_path(uri))
    .arg("--exclude")
    .arg("naming-convention")
    .arg("--json")
    .arg("-")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped()) // Redirect stderr to stdout
    .stdin(Stdio::null()) // Provide an empty input
    .spawn().unwrap();

    let out = output.stdout.take().unwrap();
    eprintln!("SLITHER STARTING");

    let mut buffer = tokio::io::BufReader::new(out);
    let mut dst = String::new();
    eprintln!("SLITHER WAITING");
    output.wait().await?;
    eprintln!("SLITHER FINISHED");
    buffer.read_to_string(&mut dst).await.unwrap();

    let json: SlitherResult =
    serde_json::from_str(&dst).expect("Failed to parse slither output");
    for detector in json.results.detectors {
        results.append(&mut crate::types::diag_from_json(detector.clone()));
    }
    eprintln!("SLITHER out: {:?}", results);

    Ok(results)
}

pub fn exec_slither(filepath: &str) -> Result<tokio::process::Child, std::io::Error> {
    Command::new("slither")
        .arg(normalize_slither_path(filepath))
        .arg("--exclude")
        .arg("naming-convention")
        .arg("--json")
        .arg("-").spawn()
}

/**
 * Find the foundry.toml config file in the given workspace using glob.
 */
pub fn find_foundry_toml_config(workspace: &str) -> Result<String, Box<dyn Error>> {
    let mut foundry_toml_path = String::new();
    for entry in glob(&format!("{}/**/foundry.toml", workspace))? {
        match entry {
                Ok(path) => {
                    foundry_toml_path = path.display().to_string();
                    break;
                }
                Err(e) => eprintln!("{:?}", e),
            }
    }
    if foundry_toml_path.is_empty() {
        return Err(Box::new(SlitherError::FoundryTomlNotFound));
    }
    Ok(foundry_toml_path)
}
