/**
 * event.rs
 * Functions to retrieve event nodes from contract AST
 * author: Leon
*/
use syn_solidity::{ItemEvent, Visit};

struct EventVisitor {
    events: Vec<ItemEvent>,
}

impl EventVisitor {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for EventVisitor {
    fn visit_item_event(&mut self, i: &ItemEvent) {
        self.events.push(i.clone());
        syn_solidity::visit::visit_item_event(self, i);
    }
}

pub fn retrieve_events_nodes(ast: syn_solidity::ItemContract) -> Vec<ItemEvent> {
    let mut visitor = EventVisitor::new();
    visitor.visit_item_contract(&ast);
    visitor.events
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
    fn test_retrieve_event_nodes_empty() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("events");
        path.push("empty.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_events_nodes(contract);
            assert_eq!(res.len(), 0);
        } else {
            panic!("Item should not have event");
        }
    }

    #[test]
    fn test_retrieve_event_nodes_one() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("events");
        path.push("one.sol");
        let source = fs::read_to_string(path).unwrap();
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let item = ast.items.first().unwrap().clone();

        if let Item::Contract(contract) = item {
            let res = retrieve_events_nodes(contract);
            assert_eq!(res.len(), 1);
        } else {
            panic!("Item should have a event");
        }
    }
}
