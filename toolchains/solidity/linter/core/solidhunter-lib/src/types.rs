use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fmt;
use colored::Colorize;

#[derive(Error, Debug)]
pub enum SolidHunterError {
    // Linter errors
    #[error("SolidHunterError: Solc error occured")]
    SolcError(#[from] solc_wrapper::SolcError),
    #[error("SolidHunterError: Something went wrong with the file during parsing")]
    ParsingError(#[from] std::io::Error),
    #[error("SolidHunterError: Something went wrong")]
    LinterError(String),
    //

    // RulesError
    #[error("SolidHunterError: IO error occured with Rules")]
    IoError(std::io::Error),
    //
}

pub type LintResult = Result<Vec<LintDiag>, SolidHunterError>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LintDiag {
    /// The range at which the message applies.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's severity. Can be omitted. If omitted it is up to the
    /// client to interpret diagnostics as error, warning, info or hint.
    pub severity: Option<Severity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's code. Can be omitted.
    pub code: Option<NumberOrString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'.
    pub source: Option<String>,

    /// The diagnostic's message.
    pub message: String,

    pub uri: Uri,

    #[serde(rename = "sourceFileContent")]
    pub source_file_content: String,
}

fn severity_to_string(severity: Option<Severity>) -> String {
    match severity {
        Some(Severity::ERROR) => "error".to_string().red(),
        Some(Severity::WARNING) => "warning".to_string().yellow(),
        Some(Severity::INFO) => "info".to_string().blue(),
        Some(Severity::HINT) => "hint".to_string().green(),
        _ => "error".to_string().red(),
    }
    .to_string()
}

impl fmt::Display for LintDiag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let padding: String;
        if self.range.start.line > 99 {
            padding = " ".repeat(0);
        } else if self.range.start.line > 9 {
            padding = " ".to_string();
        } else {
            padding = " ".repeat(2);
        }
        let line = self
            .source_file_content
            .lines()
            .nth((self.range.start.line - 1) as usize)
            .unwrap();

        write!(f, "\n{}: {}\n  --> {}:{}:{}\n   |\n{}{}|{}\n   |{}{}", severity_to_string(self.severity), self.message, self.uri, self.range.start.line, self.range.start.character, self.range.start.line, padding, line, " ".repeat(self.range.start.character as usize),
        "^".repeat(self.range.length as usize))
    }
}


////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub line: u64,
    pub character: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Severity {
    /// Reports an error.
    ERROR = 1,
    /// Reports a warning.
    WARNING = 2,
    /// Reports an information.
    INFO = 3,
    /// Reports a hint.
    HINT = 4,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
    pub length: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

type Uri = String;
