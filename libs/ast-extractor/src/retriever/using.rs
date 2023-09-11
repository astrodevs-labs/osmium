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


#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn_solidity::Item;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_using_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("using");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_usings_nodes(contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have any using directive");
        }
    }

    #[test]
    fn test_retrieve_using_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("using");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_usings_nodes(contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a a using directive");
        }
    }
}