/**
 * struct.rs
 * Functions to retrieve struct nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemStruct, Visit};

struct StructVisitor {
    structs: Vec<ItemStruct>,
}

impl StructVisitor {
    pub fn new() -> Self {
        Self {
            structs: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_item_struct(&mut self, i: &ItemStruct) {
        self.structs.push(i.clone());
        syn_solidity::visit::visit_item_struct(self, i);
    }
}

pub fn retrieve_structs_nodes(ast: syn_solidity::ItemContract) -> Vec<ItemStruct> {
    let mut visitor = StructVisitor::new();
    visitor.visit_item_contract(&ast);
    visitor.structs
}
