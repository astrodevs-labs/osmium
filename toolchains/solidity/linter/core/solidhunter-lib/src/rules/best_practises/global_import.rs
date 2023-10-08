use ast_extractor::{ImportPath, Item, Spanned};

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

// const DEFAULT_SEVERITY: &str = "warn";
const DEFAULT_MESSAGE: &str = "Import should not be global";
pub const RULE_ID: &str = "global-import";

pub struct GlobalImport {
    _data: RuleEntry,
}

impl RuleType for GlobalImport {
    fn diagnose(&self, _file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();

        let reports = check_global_import(_file);
        for report in reports.into_iter().flatten() {
            res.push(LintDiag {
                id: RULE_ID.to_string(),
                range: report,
                severity: Some(Severity::WARNING),
                code: None,
                source: None,
                message: DEFAULT_MESSAGE.to_string(),
                uri: _file.path.clone(),
                source_file_content: _file.content.clone(),
            });
        }
        println!("res: {:?}", res);
        res
    }
}

fn check_global_import(file: &SolidFile) -> Vec<Option<Range>> {
    let mut reports: Vec<Option<Range>> = Vec::new();

    file.data.items.iter().for_each(|item| {
        if let Item::Import(import) = item {
            if let ImportPath::Plain(plain) = &import.path {
                if plain.alias.is_none() {
                    reports.push(Some(Range {
                        start: Position {
                            line: plain.span().start().line,
                            character: plain.span().start().column,
                        },
                        end: Position {
                            line: plain.span().end().line,
                            character: plain.span().end().column,
                        },
                    }));
                }
            }
            if let ImportPath::Glob(glob) = &import.path {
                if glob.alias.is_none() {
                    reports.push(Some(Range {
                        start: Position {
                            line: glob.span().start().line,
                            character: glob.span().start().column,
                        },
                        end: Position {
                            line: glob.span().end().line,
                            character: glob.span().end().column,
                        },
                    }));
                }
            }
        }
    });
    reports
}

impl GlobalImport {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = GlobalImport { _data: data };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![],
        }
    }
}
