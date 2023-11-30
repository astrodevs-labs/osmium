use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::{Position, Range, NumberOrString, Diagnostic, DiagnosticSeverity as Severity};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SlitherResult {
    pub results: Vec<SlitherDetector>,
    pub success: bool,
    pub error: String,
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
    pub type_specific_fields: SlitherTypeSpecificFields,
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
    pub directives: Vec<String>,
}

pub fn from_json(json: SlitherDetector, idx: usize) -> Diagnostic {
    let lines = &json.elements[idx].source_mapping.lines;
    let start_col = json.elements[idx].source_mapping.starting_column;
    let end_col = json.elements[idx].source_mapping.ending_column;
    let range = Range {
        start: Position {
            line: lines[0] as u32,
            character: start_col as u32,
        },
        end: Position {
            line: lines[lines.len() - 1] as u32,
            character: end_col as u32,
        },
    };
    let severity = None;
    let code = Some(NumberOrString::String(json.id));
    let source = Some("slither".to_string());
    let message = json.description + "\nCheck: " + &json.check;

    Diagnostic {
        range,
        severity,
        code,
        code_description: None,
        source,
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////
