use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::Spanned;

pub const RULE_ID: &str = "func-visibility";
const MESSAGE: &str =
    "Explicitly mark visibility in function (public, private, internal, external)";

pub const DEFAULT_IGNORE_CONSTRUCTORS: bool = false;

pub struct FuncVisibility {
    ignore_constructors: bool,
    _data: RuleEntry,
}

impl FuncVisibility {
    fn create_diag(
        &self,
        location: (ast_extractor::LineColumn, ast_extractor::LineColumn),
        file: &SolidFile,
    ) -> LintDiag {
        LintDiag {
            id: RULE_ID.to_string(),
            range: Range {
                start: Position {
                    line: location.0.line,
                    character: location.0.column,
                },
                end: Position {
                    line: location.1.line,
                    character: location.1.column,
                },
            },
            message: MESSAGE.to_string(),
            severity: Some(self._data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for FuncVisibility {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);

        for contract in contracts {
            for function in ast_extractor::retriever::retrieve_functions_nodes(&contract) {
                if function.attributes.visibility().is_some()
                    || (function.kind.is_constructor() && self.ignore_constructors)
                {
                    continue;
                }
                if function.kind.is_function() {
                    res.push(
                        self.create_diag(
                            (function.kind.span().start(), function.span().end()),
                            file,
                        ),
                    );
                } else {
                    let span = function.kind.span();
                    res.push(self.create_diag((span.start(), span.end()), file));
                }
            }
        }
        res
    }
}

impl FuncVisibility {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut ignore_constructors = DEFAULT_IGNORE_CONSTRUCTORS;

        if !data.data.is_empty() {
            ignore_constructors = match data.data[0].as_bool() {
                Some(val) => val,
                None => DEFAULT_IGNORE_CONSTRUCTORS,
            };
        }
        let rule = FuncVisibility {
            ignore_constructors,
            _data: data,
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![serde_json::json!({
                    "strict": DEFAULT_IGNORE_CONSTRUCTORS,
                }),],
        }
    }
}
