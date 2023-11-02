use glob::glob;
use crate::errors::SolidHunterError;

fn parse_line (line: &str) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = glob(line) {
        for entry in entries.flatten() {
            files.push(entry.into_os_string().into_string().unwrap())
        }
    }

    files
}

pub fn get_ignored_files (filepath: &str) -> Result<Vec<String>, SolidHunterError> {
    let mut ignored_files = Vec::new();

    if !std::path::Path::new(filepath).is_file() {
        return Ok(ignored_files);
    }

    let file = std::fs::read_to_string(filepath)?;

    for line in file.lines() {
        ignored_files.append(&mut parse_line(line))
    }
    Ok(ignored_files)
}