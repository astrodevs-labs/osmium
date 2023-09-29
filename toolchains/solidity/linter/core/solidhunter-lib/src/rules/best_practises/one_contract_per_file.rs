use ast_extractor::Spanned;

use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;

pub const RULE_ID: &str = "one-contract-per-file";
const MESSAGE: &str = "Only one contract per file";

pub struct OneContractPerFile {
    data: RuleEntry,
}

impl OneContractPerFile {
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
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }
}

impl RuleType for OneContractPerFile {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let contracts = ast_extractor::retriever::retrieve_contract_nodes(&file.data);
        let contract_count = contracts.len();

        if contract_count <= 1 {
            return res;
        }

        for contract in contracts {
            let span = contract.name.span();
            res.push(self.create_diag((span.start(), span.end()), file));
        }
        res
    }
}

impl OneContractPerFile {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let rule = OneContractPerFile { data };
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
