use crate::error::SlitherError;
use crate::SlitherData;
use std::error::Error;
use std::process::Command as StdCommand;
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

pub fn parse_foundry_toml(foundry: String, state: &mut SlitherData) {
    let foundry: toml::Value = match foundry.parse() {
        Ok(foundry) => foundry,
        Err(e) => {
            eprintln!("Error parsing foundry.toml: {}", e);
            return;
        }
    };

    let libs = foundry["profile"]["default"]["libs"].as_array();
    match libs {
        Some(libs) => {
            for lib in libs {
                state.libs_paths.push(lib.to_string());
            }
        }
        None => {
            state
                .libs_paths
                .push(foundry["profile"]["default"]["libs"].to_string());
        }
    }
    let src = foundry["profile"]["default"]["src"].as_array();
    match src {
        Some(src) => {
            for src in src {
                state.src_paths.push(src.to_string());
            }
        }
        None => {
            state
                .src_paths
                .push(foundry["profile"]["default"]["src"].to_string());
        }
    }
    let tests = foundry["profile"]["default"]["test"].as_array();
    match tests {
        Some(tests) => {
            for test in tests {
                state.tests_paths.push(test.to_string());
            }
        }
        None => {
            state
                .tests_paths
                .push(foundry["profile"]["default"]["test"].to_string());
        }
    }
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
