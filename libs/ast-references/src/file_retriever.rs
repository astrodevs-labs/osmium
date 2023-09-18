/**
 * file_retriever.rs
 * Function to retrieve a file reference from AST
 * author: ByFish
 */
use crate::types::contract_reference::ContractReference;
use crate::types::file_reference::FileReference;
use crate::types::location::{Bound, Location};
use crate::types::struct_reference::StructReference;
use proc_macro2::TokenStream;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;
use syn_solidity::{ItemContract, ItemStruct, Visit};

struct FileVisitor {
    file_reference: Rc<RefCell<FileReference>>,
    current_contract: Option<Rc<RefCell<ContractReference>>>,
}

impl FileVisitor {
    pub fn new(path: String) -> Self {
        Self {
            file_reference: Rc::new(RefCell::new(FileReference::new(path.to_string()))),
            current_contract: None,
        }
    }
}

impl<'ast> Visit<'ast> for FileVisitor {
    fn visit_item_contract(&mut self, i: &ItemContract) {
        let contract_reference = ContractReference::new(
            i.name.to_string(),
            Location::new(
                self.file_reference.borrow_mut().path.clone(),
                Bound::new(
                    i.brace_token.span.join().start().line as u32,
                    i.brace_token.span.join().start().column as u32,
                ),
                Bound::new(
                    i.brace_token.span.join().end().line as u32,
                    i.brace_token.span.join().end().column as u32,
                ),
            ),
            &self.file_reference,
        );
        self.file_reference
            .borrow_mut()
            .add_contract(contract_reference);
        self.current_contract = Some(
            self.file_reference
                .borrow()
                .contracts
                .last()
                .unwrap()
                .clone(),
        );
        syn_solidity::visit::visit_item_contract(self, i);
        self.current_contract = None;
    }
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let struct_reference = StructReference::new(
            i.name.to_string(),
            Location::new(
                self.file_reference.borrow_mut().path.clone(),
                Bound::new(
                    i.brace_token.span.join().start().line as u32,
                    i.brace_token.span.join().start().column as u32,
                ),
                Bound::new(
                    i.brace_token.span.join().end().line as u32,
                    i.brace_token.span.join().end().column as u32,
                ),
            ),
            None,
            Some(&self.file_reference),
        );
        if self.current_contract.is_some() {
            self.current_contract
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_struct(&Rc::new(RefCell::new(struct_reference)));
        } else {
            self.file_reference
                .borrow_mut()
                .add_struct(struct_reference);
        }
        syn_solidity::visit::visit_item_struct(self, i)
    }
}

pub fn retrieve_file_reference_from_path(path: String) -> Rc<RefCell<FileReference>> {
    let source = fs::read_to_string(path.to_string()).unwrap();
    let tokens = TokenStream::from_str(source.as_str()).unwrap();
    let ast = syn_solidity::parse2(tokens).unwrap();
    let mut visitor = FileVisitor::new(path.to_string());
    visitor.visit_file(&ast);
    visitor.file_reference
}
