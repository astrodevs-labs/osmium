/**
 * udt.rs
 * Functions to retrieve udt nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemUdt, Visit};

struct UdtVisitor {
    udts: Vec<ItemUdt>,
}

impl UdtVisitor {
    pub fn new() -> Self {
        Self { udts: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for UdtVisitor {
    fn visit_item_udt(&mut self, i: &ItemUdt) {
        self.udts.push(i.clone());
        syn_solidity::visit::visit_item_udt(self, i);
    }
}

pub fn retrieve_udts_nodes(ast: syn_solidity::ItemContract) -> Vec<ItemUdt> {
    let mut visitor = UdtVisitor::new();
    visitor.visit_item_contract(&ast);
    visitor.udts
}
