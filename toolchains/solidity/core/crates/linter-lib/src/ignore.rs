use crate::errors::SolidHunterError;
use glob::glob;
use std::path::Path;

fn parse_line(line: &str, path: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let line = line.replace("./", "");
    if let Some(parent) = path.parent() {
        if let Some(filepath) = parent.join(line).to_str() {
            if let Ok(entries) = glob(filepath) {
                for entry in entries.flatten() {
                    files.push(entry.into_os_string().into_string().unwrap())
                }
            }
        }
    }

    files
}

pub fn get_ignored_files(filepath: &str) -> Result<Vec<String>, SolidHunterError> {
    let mut ignored_files = Vec::new();
    let path = Path::new(filepath);

    if !path.is_file() {
        return Ok(ignored_files);
    }

    let file = std::fs::read_to_string(path)?;

    for line in file.lines() {
        ignored_files.append(&mut parse_line(line, path))
    }
    Ok(ignored_files)
}
