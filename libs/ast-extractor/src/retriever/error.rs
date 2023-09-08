/**
 * error.rs
 * Functions to retrieve error nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemError, Visit};

struct ErrorVisitor {
    errors: Vec<ItemError>,
}

impl ErrorVisitor {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ErrorVisitor {
    fn visit_item_error(&mut self, i: &ItemError) {
        self.errors.push(i.clone());
        syn_solidity::visit::visit_item_error(self, i);
    }
}

pub fn retrieve_errors_nodes(ast: syn_solidity::ItemContract) -> Vec<ItemError> {
    let mut visitor = ErrorVisitor::new();
    visitor.visit_item_contract(&ast);
    visitor.errors
}
