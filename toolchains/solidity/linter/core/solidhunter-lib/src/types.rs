use crate::errors::SolidHunterError;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::cmp::min;
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

fn compute_format_line_padding(line: usize) -> String {
    let padding: String;
    if line > 99 {
        padding = " ".repeat(0);
    } else if line > 9 {
        padding = " ".to_string();
    } else {
        padding = " ".repeat(2);
    }
    padding
}

fn try_trim_max_offset(line: &str, max_offset: usize) -> (&str, usize) {
    let mut offset: usize = 0;

    for (i, c) in line.chars().enumerate() {
        if i >= max_offset {
            break;
        }
        if c.is_whitespace() {
            offset += 1;
        }
    }
    (&line[offset..], offset)
}

impl LintDiag {
    fn format_highlighted_lines(&self) -> String {
        let mut formatted = "   |\n".to_string();
        let first_line = self
            .source_file_content
            .lines()
            .nth(self.range.start.line - 1)
            .unwrap();
        let trimmed_first_line = first_line.trim_start();
        let max_offset = first_line.len() - trimmed_first_line.len();

        for line_nb in self.range.start.line..self.range.end.line + 1 {
            let line = self.source_file_content.lines().nth(line_nb - 1).unwrap();
            let (trimmed_line, offset) = try_trim_max_offset(line, max_offset);
            let mut higlight_length = trimmed_line.len();

            if self.range.start.line == self.range.end.line {
                higlight_length = self.range.end.character - self.range.start.character;
            } else if line_nb == self.range.start.line {
                higlight_length = trimmed_line.len() - (self.range.start.character - offset);
            } else if line_nb == self.range.end.line {
                higlight_length = trimmed_line.len()
                    - (self.range.end.character - min(offset, trimmed_line.len()));
            }

            formatted = format!(
                "{}{}{}|    {}\n   |    {}{}\n",
                formatted,
                line_nb,
                compute_format_line_padding(line_nb),
                trimmed_line,
                " ".repeat(if line_nb == self.range.start.line {
                    self.range.start.character - offset
                } else {
                    0
                }),
                "^".repeat(higlight_length)
            );
        }
        formatted
    }
}

impl fmt::Display for LintDiag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{}: {}\n  --> {}:{}:{}\n   |\n{}",
            severity_to_string(self.severity),
            self.message,
            self.uri,
            self.range.start.line,
            self.range.start.character,
            self.format_highlighted_lines()
        )
    }
}

////////////////////////////////////////////////////////////
/////////////////// RELATED TYPES: /////////////////////////
////////////////////////////////////////////////////////////

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

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.character == other.character
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub line: usize,
    pub character: usize,
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
}

impl Range {
    // Compute the number of characters between the start and end of the range
    pub fn compute_length(&self, content: &str) -> usize {
        if self.start.line == self.end.line {
            self.end.character - self.start.character
        } else {
            let mut length = 0;
            let mut line = self.start.line;
            let mut character = self.start.character;
            while line < self.end.line {
                let line_content = content.lines().nth(line - 1).unwrap();
                length += line_content.len() + 1 - character;
                line += 1;
                character = 0;
            }
            length += self.end.character - character;
            length
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

type Uri = String;
