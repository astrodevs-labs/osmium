/**
 * file_retriever.rs
 * Function to retrieve a file reference from AST
 * author: ByFish
 */
use crate::types::contract_reference::ContractReference;
use crate::types::event_reference::EventReference;
use crate::types::file_reference::FileReference;
use crate::types::struct_reference::StructReference;
use crate::types::function_reference::FunctionReference;
use crate::types::location::{Bound, Location};
use proc_macro2::TokenStream;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;
use syn_solidity::{ItemContract, ItemEvent, ItemFunction, ItemStruct, Spanned, Visit};

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
                    i.name.span().start().line as u32,
                    i.name.span().start().column as u32,
                ),
                Bound::new(
                    i.name.span().end().line as u32,
                    i.name.span().end().column as u32,
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

    //fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
    //    let enum_reference = EnumReference::new(
    //        i.name.to_string(),
    //        Location::new(
    //            self.file_reference.borrow_mut().path.clone(),
    //            Bound::new(
    //                i.name.span().start().line as u32,
    //                i.name.span().start().column as u32,
    //            ),
    //            Bound::new(
    //                i.name.span().start().line as u32,
    //                i.name.span().start().column as u32,
    //            ),
    //        ),
    //        self.current_contract.as_ref(),
    //        Some(&self.file_reference),
    //    );
    //    if self.current_contract.is_some() {
    //        self.current_contract
    //            .as_ref()
    //            .unwrap()
    //            .borrow_mut()
    //            .add_enum(&Rc::new(RefCell::new(enum_reference)));
    //    } else {
    //        self.file_reference.borrow_mut().add_enum(enum_reference);
    //    }
    //    syn_solidity::visit::visit_item_enum(self, i)
    //}

    fn visit_item_event(&mut self, i: &'ast ItemEvent) {
        let event_reference = EventReference::new(
            i.name.to_string(),
            Location::new(
                self.file_reference.borrow_mut().path.clone(),
                Bound::new(
                    i.name.span().start().line as u32,
                    i.name.span().start().column as u32,
                ),
                Bound::new(
                    i.name.span().end().line as u32,
                    i.name.span().end().column as u32,
                ),
            ),
            self.current_contract.as_ref(),
            Some(&self.file_reference),
        );
        if self.current_contract.is_some() {
            self.current_contract
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_event(&Rc::new(RefCell::new(event_reference)));
        } else {
            self.file_reference.borrow_mut().add_event(event_reference);
        }
        syn_solidity::visit::visit_item_event(self, i)
    }

    fn visit_item_function(&mut self, i: &'ast ItemFunction) {
        if self.current_contract.is_some() {
            let function_reference = FunctionReference::new(
                i.name.as_ref().unwrap().0.to_string().clone(),
                i.kind.clone(),
                Location::new(
                    self.file_reference.borrow_mut().path.clone(),
                    Bound::new(
                        i.name.span().start().line as u32,
                        i.name.span().start().column as u32,
                    ),
                    Bound::new(
                        i.name.span().end().line as u32,
                        i.name.span().end().column as u32,
                    ),
                ),
                self.current_contract.as_ref().unwrap(),
            );
            self.current_contract.as_ref().unwrap().borrow_mut().add_function(&Rc::new(RefCell::new(function_reference)))
        }
        syn_solidity::visit::visit_item_function(self, i)
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let struct_reference = StructReference::new(
            i.name.to_string(),
            Location::new(
                self.file_reference.borrow_mut().path.clone(),
                Bound::new(
                    i.name.span().start().line as u32,
                    i.name.span().start().column as u32,
                ),
                Bound::new(
                    i.name.span().end().line as u32,
                    i.name.span().end().column as u32,
                ),
            ),
            self.current_contract.as_ref(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrieve_contract_nodes_empty() {
        retrieve_file_reference_from_path("C:\\Users\\byfish\\Desktop\\DEV\\osmium\\libs\\ast-extractor\\tests\\files\\contracts\\two.sol".to_string());
        assert_eq!(1, 0)
    }
}
