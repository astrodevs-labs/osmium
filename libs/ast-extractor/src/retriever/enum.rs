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

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn_solidity::Item;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_enum_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("enums");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_enums_nodes(contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item shouldn't have enum");
        }
    }

    #[test]
    fn test_retrieve_enum_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("enums");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let contract = ast.items.iter().find(|i| match i {
            Item::Contract(_) => true,
            _ => false,
        }).unwrap().clone();

        if let Item::Contract(contract) = contract {
            let res = retrieve_enums_nodes(contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a contract");
        }
    }
}
