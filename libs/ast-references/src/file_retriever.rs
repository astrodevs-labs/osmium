/**
 * file_retriever.rs
 * Function to retrieve a file reference from AST
 * author: ByFish
 */
use crate::types::contract_reference::ContractReference;
use crate::types::enum_reference::EnumReference;
use crate::types::error_reference::ErrorReference;
use crate::types::event_reference::EventReference;
use crate::types::file_reference::FileReference;
use crate::types::function_reference::FunctionReference;
use crate::types::location::{Bound, Location};
use crate::types::struct_reference::StructReference;
use crate::types::variable_reference::VariableReference;
use proc_macro2::TokenStream;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;
use ast_extractor::{
    ItemContract, ItemEnum, ItemError, ItemEvent, ItemFunction, ItemStruct, Spanned,
    VariableDefinition, Visit,
};

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
    fn visit_variable_definition(&mut self, i: &'ast VariableDefinition) {
        let variable_reference = VariableReference::new(
            i.name.0.to_string().clone(),
            i.ty.clone(),
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
                .add_property(&Rc::new(RefCell::new(variable_reference)));
        } else {
            self.file_reference
                .borrow_mut()
                .add_variable(variable_reference);
        }
        ast_extractor::visit::visit_variable_definition(self, i)
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let enum_reference = EnumReference::new(
            i.name.to_string(),
            Location::new(
                self.file_reference.borrow_mut().path.clone(),
                Bound::new(
                    i.name.span().start().line as u32,
                    i.name.span().start().column as u32,
                ),
                Bound::new(
                    i.name.span().start().line as u32,
                    i.name.span().start().column as u32,
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
                .add_enum(&Rc::new(RefCell::new(enum_reference)));
        } else {
            self.file_reference.borrow_mut().add_enum(enum_reference);
        }
        ast_extractor::visit::visit_item_enum(self, i)
    }

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
        ast_extractor::visit::visit_item_contract(self, i);
        self.current_contract = None;
    }

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
        ast_extractor::visit::visit_item_event(self, i)
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
            self.current_contract
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_function(&Rc::new(RefCell::new(function_reference)));
        }
        ast_extractor::visit::visit_item_function(self, i);
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
        ast_extractor::visit::visit_item_struct(self, i)
    }

    fn visit_item_error(&mut self, i: &'ast ItemError) {
        let error_reference = ErrorReference::new(
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
                .add_error(&Rc::new(RefCell::new(error_reference)));
        } else {
            self.file_reference.borrow_mut().add_error(error_reference);
        }
        ast_extractor::visit::visit_item_error(self, i)
    }
}

pub fn retrieve_file_reference_from_path(path: String) -> Rc<RefCell<FileReference>> {
    let source = fs::read_to_string(path.to_string()).unwrap();
    let tokens = TokenStream::from_str(source.as_str()).unwrap();
    let ast = ast_extractor::parse2(tokens).unwrap();
    let mut visitor = FileVisitor::new(path.to_string());
    visitor.visit_file(&ast);
    visitor.file_reference
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrieve_contract_variables() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("variables");
        path.push("contract.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let contract_ref = ContractReference::new(
            "Good".to_string(),
            Location::new(path_str, Bound { line: 1, column: 1 }, Bound::new(1, 10)),
            &visitor.file_reference,
        );
        visitor
            .file_reference
            .borrow_mut()
            .add_contract(contract_ref);
        visitor.current_contract = Some(visitor.file_reference.borrow().contracts[0].clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let contract = file.items.iter().find(|item| match item {
            ast_extractor::Item::Contract(_) => true,
            _ => false,
        });
        let contract = match contract {
            Some(ast_extractor::Item::Contract(contract)) => contract,
            _ => panic!("No contract found"),
        };
        let variables = contract.body.iter().find(|item| match item {
            ast_extractor::Item::Variable(_) => true,
            _ => false,
        });
        let variables = match variables {
            Some(ast_extractor::Item::Variable(variables)) => variables,
            _ => panic!("No variables found"),
        };
        visitor.visit_variable_definition(variables);
        assert_eq!(
            visitor.file_reference.borrow().contracts[0]
                .borrow()
                .properties
                .len(),
            1
        );
    }

    #[test]
    fn test_retrieve_file_variables() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("variables");
        path.push("file.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let variable = file.items.iter().find(|item| match item {
            ast_extractor::Item::Variable(_) => true,
            _ => false,
        });
        let variable = match variable {
            Some(ast_extractor::Item::Variable(var)) => var,
            _ => panic!("Expect variable declaration"),
        };
        visitor.visit_variable_definition(variable);
        assert_eq!(visitor.file_reference.borrow().variables.len(), 1);
    }

    #[test]
    fn test_retrieve_contract_enums() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("enums");
        path.push("contract.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let contract_ref = ContractReference::new(
            "Good".to_string(),
            Location::new(path_str, Bound { line: 1, column: 1 }, Bound::new(1, 10)),
            &visitor.file_reference,
        );
        visitor
            .file_reference
            .borrow_mut()
            .add_contract(contract_ref);
        visitor.current_contract = Some(visitor.file_reference.borrow().contracts[0].clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let contract = file.items.iter().find(|item| match item {
            ast_extractor::Item::Contract(_) => true,
            _ => false,
        });
        let contract = match contract {
            Some(ast_extractor::Item::Contract(contract)) => contract,
            _ => panic!("No contract found"),
        };
        let enums = contract.body.iter().find(|item| match item {
            ast_extractor::Item::Enum(_) => true,
            _ => false,
        });
        let enums = match enums {
            Some(ast_extractor::Item::Enum(enums)) => enums,
            _ => panic!("No enums found"),
        };
        visitor.visit_item_enum(enums);
        assert_eq!(
            visitor.file_reference.borrow().contracts[0]
                .borrow()
                .enums
                .len(),
            1
        );
    }

    #[test]
    fn test_retrieve_file_enums() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("enums");
        path.push("file.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let enums = file.items.iter().find(|item| match item {
            ast_extractor::Item::Enum(_) => true,
            _ => false,
        });
        let enums = match enums {
            Some(ast_extractor::Item::Enum(var)) => var,
            _ => panic!("Expect enums declaration"),
        };
        visitor.visit_item_enum(enums);
        assert_eq!(visitor.file_reference.borrow().enums.len(), 1);
    }

    #[test]
    fn test_retrieve_contract_nodes_empty() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("contracts");
        path.push("file.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let contracts = file.items.iter().find(|item| match item {
            ast_extractor::Item::Contract(_) => true,
            _ => false,
        });
        let contracts = match contracts {
            Some(ast_extractor::Item::Contract(var)) => var,
            _ => panic!("Expect contracts declaration"),
        };
        visitor.visit_item_contract(contracts);
        assert_eq!(visitor.file_reference.borrow().contracts.len(), 1);
    }

    #[test]
    fn test_retrieve_contract_event() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("events");
        path.push("contract.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let contract = file.items.iter().find(|item| match item {
            ast_extractor::Item::Contract(_) => true,
            _ => false,
        });
        let contract = match contract {
            Some(ast_extractor::Item::Contract(contract)) => contract,
            _ => panic!("No contract found"),
        };
        let events = contract.body.iter().find(|item| match item {
            ast_extractor::Item::Event(_) => true,
            _ => false,
        });
        let events = match events {
            Some(ast_extractor::Item::Event(var)) => var,
            _ => panic!("Expect events declaration"),
        };
        visitor.visit_item_event(events);
        assert_eq!(visitor.file_reference.borrow().events.len(), 1);
    }

    #[test]
    fn test_retrieve_functions() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("functions");
        path.push("contract.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let contract_ref = ContractReference::new(
            "Good".to_string(),
            Location::new(path_str, Bound { line: 1, column: 1 }, Bound::new(10, 100)),
            &visitor.file_reference,
        );
        visitor
            .file_reference
            .borrow_mut()
            .add_contract(contract_ref);
        visitor.current_contract = Some(visitor.file_reference.borrow().contracts[0].clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let contract = file.items.iter().find(|item| match item {
            ast_extractor::Item::Contract(_) => true,
            _ => false,
        });
        let contract = match contract {
            Some(ast_extractor::Item::Contract(contract)) => contract,
            _ => panic!("No contract found"),
        };
        let function = contract.body.iter().find(|item| match item {
            ast_extractor::Item::Function(_) => true,
            _ => false,
        });
        let function = match function {
            Some(ast_extractor::Item::Function(function)) => function,
            _ => panic!("No function found"),
        };
        visitor.visit_item_function(function);
        assert_eq!(
            visitor.file_reference.borrow().contracts[0]
                .borrow()
                .functions
                .len(),
            1
        );
    }

    #[test]
    fn test_retrieve_contract_structs() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("structs");
        path.push("contract.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let contract_ref = ContractReference::new(
            "Good".to_string(),
            Location::new(path_str, Bound { line: 1, column: 1 }, Bound::new(1, 10)),
            &visitor.file_reference,
        );
        visitor
            .file_reference
            .borrow_mut()
            .add_contract(contract_ref);
        visitor.current_contract = Some(visitor.file_reference.borrow().contracts[0].clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let contract = file.items.iter().find(|item| match item {
            ast_extractor::Item::Contract(_) => true,
            _ => false,
        });
        let contract = match contract {
            Some(ast_extractor::Item::Contract(contract)) => contract,
            _ => panic!("No contract found"),
        };
        let structs = contract.body.iter().find(|item| match item {
            ast_extractor::Item::Struct(_) => true,
            _ => false,
        });
        let structs = match structs {
            Some(ast_extractor::Item::Struct(structs)) => structs,
            _ => panic!("No structs found"),
        };
        visitor.visit_item_struct(structs);
        assert_eq!(
            visitor.file_reference.borrow().contracts[0]
                .borrow()
                .structs
                .len(),
            1
        );
    }

    #[test]
    fn test_retrieve_file_structs() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("structs");
        path.push("file.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let structs = file.items.iter().find(|item| match item {
            ast_extractor::Item::Struct(_) => true,
            _ => false,
        });
        let structs = match structs {
            Some(ast_extractor::Item::Struct(var)) => var,
            _ => panic!("Expect structs declaration"),
        };
        visitor.visit_item_struct(structs);
        assert_eq!(visitor.file_reference.borrow().structs.len(), 1);
    }

    #[test]
    fn test_retrieve_errors() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("errors");
        path.push("contract.sol");
        let path_str = path.to_str().unwrap().to_string();
        let source = fs::read_to_string(&path).unwrap();

        let mut visitor = FileVisitor::new(path_str.clone());
        let file = ast_extractor::extract::extract_ast_from_content(&source).unwrap();
        let errors = file.items.iter().find(|item| match item {
            ast_extractor::Item::Error(_) => true,
            _ => false,
        });
        let errors = match errors {
            Some(ast_extractor::Item::Error(var)) => var,
            _ => panic!("Expect errors declaration"),
        };
        visitor.visit_item_error(errors);
        assert_eq!(visitor.file_reference.borrow().errors.len(), 1);
    }
}
