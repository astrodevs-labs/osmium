use crate::error::SolidHunterError;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

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

    pub id: String,

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

        write!(
            f,
            "\n{}: {}\n  --> {}:{}:{}\n   |\n{}{}|{}\n   |{}{}",
            severity_to_string(self.severity),
            self.message,
            self.uri,
            self.range.start.line,
            self.range.start.character,
            self.range.start.line,
            padding,
            line,
            " ".repeat(self.range.start.character as usize),
            "^".repeat(self.range.length as usize)
        )
    }
}

////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.character == other.character
    }
}

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

impl Range {
    // Compue the number of characters between the start and end of the range
    pub fn compute_length(&mut self, content: &str) {
        if self.start.line == self.end.line {
            self.length = self.end.character - self.start.character;
        } else {
            let mut length = 0;
            let mut line = self.start.line;
            let mut character = self.start.character;
            while line < self.end.line {
                let line_content = content.lines().nth(line as usize - 1).unwrap();
                length += line_content.len() + 1 - character as usize;
                line += 1;
                character = 0;
            }
            let line_content = content.lines().nth(line as usize - 1).unwrap();
            length += self.end.character as usize - character as usize;
            self.length = length as u64;
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

type Uri = String;
