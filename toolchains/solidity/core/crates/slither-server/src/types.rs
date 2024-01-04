use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity as Severity, Position, Range};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherResult {
    pub results: SlitherResults,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherResults {
    pub detectors: Vec<SlitherDetector>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherDetector {
    pub elements: Vec<SlitherElement>,
    pub description: String,
    pub check: String,
    pub impact: String,
    pub id: String,
    pub confidence: String,
    pub markdown: String,
    pub first_markdown_element: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherElement {
    pub source_mapping: SlitherSourceMapping,

    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub type_specific_fields: Option<SlitherTypeSpecificFields>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherSourceMapping {
    pub filename_absolute: String,
    pub filename_relative: String,
    pub filename_short: String,
    pub is_dependency: bool,
    pub lines: Vec<usize>,
    pub starting_column: usize,
    pub ending_column: usize,
    pub length: usize,
    pub start: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherTypeSpecificFields {
    pub directive: Option<Vec<String>>,
    pub signature: Option<String>,
    pub additional_fields: Option<SlitherAdditionalFields>,
    pub parent: Option<SlitherParent>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherParent {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub type_specific_fields: Option<Box<SlitherTypeSpecificFields>>,
    pub source_mapping: Option<SlitherSourceMapping>,
    pub signature: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherAdditionalFields {
    pub underlying_type: Option<String>,
    pub visibility: Option<String>,
    pub variable_name: Option<String>,
}

pub fn diag_from_json(json: SlitherDetector) -> Vec<Diagnostic> {
    let mut results: Vec<Diagnostic> = Vec::new();

    for idx in 0..json.elements.len() {
        if json.elements[idx].source_mapping.lines.is_empty()
            || json.elements[idx].type_ == "contract"
        {
            continue;
        }
        let lines = &json.elements[idx].source_mapping.lines;
        let start_col = json.elements[idx].source_mapping.starting_column;
        let end_col = json.elements[idx].source_mapping.ending_column;
        let range = Range {
            start: Position {
                line: lines[0] as u32 - 1,
                character: start_col as u32 - 1,
            },
            end: Position {
                line: lines[lines.len() - 1] as u32 - 1,
                character: end_col as u32,
            },
        };

        let severity = match json.impact.as_str() {
            "High" => Severity::ERROR,
            "Medium" => Severity::WARNING,
            "Low" => Severity::HINT,
            "Informational" => Severity::INFORMATION,
            _ => Severity::ERROR,
        };

        results.push(Diagnostic {
            range,
            severity: Some(severity),
            code: None,
            code_description: None,
            source: Some("osmium-slither".to_string()),
            message: json.description.to_string() + "\nCheck: " + &json.check,
            related_information: None,
            tags: None,
            data: None,
        });
    }

    results
}

////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////
