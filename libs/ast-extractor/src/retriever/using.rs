/**
 * using.rs
 * Functions to retrieve using nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemUsing, Visit};

struct UsingVisitor {
    usings: Vec<ItemUsing>,
}

impl UsingVisitor {
    pub fn new() -> Self {
        Self { usings: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for UsingVisitor {
    fn visit_item_using(&mut self, i: &ItemUsing) {
        self.usings.push(i.clone());
        syn_solidity::visit::visit_item_using(self, i);
    }
}

pub fn retrieve_usings_nodes(ast: syn_solidity::ItemContract) -> Vec<ItemUsing> {
    let mut visitor = UsingVisitor::new();
    visitor.visit_item_contract(&ast);
    visitor.usings
}
