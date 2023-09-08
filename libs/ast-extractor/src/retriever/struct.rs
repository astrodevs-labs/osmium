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

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn_solidity::Item;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_struct_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("structs");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_structs_nodes(contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have struct");
        }
    }

    #[test]
    fn test_retrieve_struct_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("structs");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_structs_nodes(contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a struct");
        }
    }
}
