use crate::errors::SolidHunterError;
use crate::rules::create_default_rules;
use crate::rules::types::*;

pub fn create_rules_file(path: &str) {
    let rules = Rules {
        name: "solidhunter".to_string(),
        includes: vec![],
        plugins: vec![],
        rules: create_default_rules(),
    };
    let serialized = serde_json::to_string_pretty(&rules).unwrap();

    std::fs::write(path, serialized).unwrap();
}

pub fn parse_rules(path: &str) -> Result<Rules, SolidHunterError> {
    if !std::path::Path::new(&path).is_file() {
        return Err(SolidHunterError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Rules file not found",
        )));
    }
    let file = std::fs::read_to_string(path)?;
    let parsed: Rules = serde_json::from_str(&file)?;

    Ok(parsed)
}
