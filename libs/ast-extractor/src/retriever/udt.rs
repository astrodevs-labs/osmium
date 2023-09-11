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

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn_solidity::Item;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_udt_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("udt");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_udts_nodes(contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have any user defined type");
        }
    }

    #[test]
    fn test_retrieve_udt_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("udt");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_udts_nodes(contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have atleast 1 user defined type");
        }
    }
}