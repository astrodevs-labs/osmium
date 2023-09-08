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
