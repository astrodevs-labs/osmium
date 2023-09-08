/**
 * enum.rs
 * Functions to retrieve enum nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemEnum, Visit};

struct EnumVisitor {
    enums: Vec<ItemEnum>,
}

impl EnumVisitor {
    pub fn new() -> Self {
        Self { enums: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_item_enum(&mut self, i: &ItemEnum) {
        self.enums.push(i.clone());
        syn_solidity::visit::visit_item_enum(self, i);
    }
}

pub fn retrieve_enums_nodes(ast: syn_solidity::ItemContract) -> Vec<ItemEnum> {
    let mut visitor = EnumVisitor::new();
    visitor.visit_item_contract(&ast);
    visitor.enums
}
