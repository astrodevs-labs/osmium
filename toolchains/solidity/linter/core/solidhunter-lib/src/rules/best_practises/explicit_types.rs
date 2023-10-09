use crate::linter::SolidFile;
use crate::rules::types::*;
use crate::types::*;
use ast_extractor::*;
use serde_json::Value;

pub const RULE_ID: &str = "explicit-types";

const DEFAULT_RULE: &str = "explicit";

pub struct ExplicitTypes {
    rule: String,
    data: RuleEntry,
}

pub struct ExplicitTypesVisitor {
    type_names: Vec<Type>,
    type_inits: Vec<SolIdent>,
}

impl<'ast> Visit<'ast> for ExplicitTypesVisitor {
    fn visit_type(&mut self, ty: &'ast Type) {
        match ty {
            Type::Int(_, _) => {
                self.type_names.push(ty.clone());
            }
            Type::Uint(_, _) => {
                self.type_names.push(ty.clone());
            }
            _ => visit::visit_type(self, ty),
        }
    }

    fn visit_variable_definition(&mut self, var: &'ast VariableDefinition) {
        if let Some((_, expr)) = &var.initializer {
            visit::visit_expr(self, expr);
        }
        visit::visit_variable_definition(self, var);
    }
}

impl ExplicitTypes {
    fn create_diag(&self, file: &SolidFile, ty: Box<dyn Spanned>, line: &str) -> LintDiag {
        let rule_formated = match self.rule.as_str() {
            "explicit" => "Explicit",
            "implicit" => "Implicit",
            _ => "Explicit",
        };
        LintDiag {
            range: Range {
                start: Position {
                    line: ty.span().start().line,
                    character: ty.span().start().column,
                },
                end: Position {
                    line: ty.span().end().line,
                    character: ty.span().end().column,
                },
            },
            id: RULE_ID.to_string(),
            message: format!("{} types are not allowed: {}", rule_formated, line.len()),
            severity: Some(self.data.severity),
            code: None,
            source: None,
            uri: file.path.clone(),
            source_file_content: file.content.clone(),
        }
    }

    fn _check_type(&self, ty: &str, file: &SolidFile, span: Box<dyn Spanned>) -> Option<LintDiag> {
        if (self.rule == "explicit" && (ty == "int" || ty == "uint"))
            || (self.rule == "implicit" && (ty != "int" && ty != "uint"))
        {
            return Some(self.create_diag(file, span, ty));
        }
        None
    }
}

impl RuleType for ExplicitTypes {
    fn diagnose(&self, file: &SolidFile, _files: &[SolidFile]) -> Vec<LintDiag> {
        let mut res = Vec::new();
        let mut visitor = ExplicitTypesVisitor {
            type_names: vec![],
            type_inits: vec![],
        };
        visitor.visit_file(&file.data);
        for ty in visitor.type_names {
            if let Some(diag) = self._check_type(&ty.to_string(), file, Box::new(ty.clone())) {
                res.push(diag);
            }
        }
        for ty in visitor.type_inits {
            if let Some(diag) = self._check_type(&ty.to_string(), file, Box::new(ty.clone())) {
                res.push(diag);
            }
        }
        res
    }
}

impl ExplicitTypes {
    pub(crate) fn create(data: RuleEntry) -> Box<dyn RuleType> {
        let mut value;
        if !data.data.is_empty(){
            value = match &data.data[0] {
                Value::String(val) => val.as_str(),
                _ => DEFAULT_RULE,
            }
        }
        else {
            value = DEFAULT_RULE;
        }
        let rule = ExplicitTypes {
            rule: value.to_string(),
            data,
        };
        Box::new(rule)
    }

    pub(crate) fn create_default() -> RuleEntry {
        RuleEntry {
            id: RULE_ID.to_string(),
            severity: Severity::WARNING,
            data: vec![serde_json::Value::String(DEFAULT_RULE.to_string())],
        }
    }
}
