use std::cell::RefCell;
/**
 * FileReference.rs
 * Definition of FileReference struct
 * author: 0xMemoryGrinder
*/

use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use super::contract_reference::ContractReference;
use super::struct_reference::StructReference;
use super::enum_reference::EnumReference;
use super::error_reference::ErrorReference;
use super::event_reference::EventReference;
use super::variable_reference::VariableReference;

/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct FileReference {
    pub path: String,
    pub contracts: Vec<Rc<RefCell<ContractReference>>>,
    pub structs: Vec<Rc<RefCell<StructReference>>>,
    pub enums: Vec<Rc<RefCell<EnumReference>>>,
    pub errors: Vec<Rc<RefCell<ErrorReference>>>,
    pub events: Vec<Rc<RefCell<EventReference>>>,
    pub variables: Vec<Rc<RefCell<VariableReference>>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

impl FileReference {
    pub fn new(path: String) -> FileReference {
        FileReference {
            path: path,
            contracts: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            errors: Vec::new(),
            events: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn add_contract(&mut self, contract: ContractReference) {
        self.contracts.push(Rc::new(RefCell::new(contract)));
    }

    pub fn add_struct(&mut self, strct: StructReference) {
        self.structs.push(Rc::new(RefCell::new(strct)));
    }

    pub fn add_enum(&mut self, enm: EnumReference) {
        self.enums.push(Rc::new(RefCell::new(enm)));
    }

    pub fn add_error(&mut self, error: ErrorReference) {
        self.errors.push(Rc::new(RefCell::new(error)));
    }

    pub fn add_event(&mut self, event: EventReference) {
        self.events.push(Rc::new(RefCell::new(event)));
    }

    pub fn add_variable(&mut self, variable: VariableReference) {
        self.variables.push(Rc::new(RefCell::new(variable)));
    }
}

impl fmt::Display for FileReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File {}", self.path)
    }
}

impl fmt::Debug for FileReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File {}", self.path)
    }
}

impl PartialEq for FileReference {
    fn eq(&self, other: &FileReference) -> bool {
        self.path == other.path
    }
}

impl Eq for FileReference {}

impl Hash for FileReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}


/******************************************************************************
 *                                  Tests                                     *
 *****************************************************************************/

 #[cfg(test)]
 mod tests {
    use std::cell::RefCell; 
    use proc_macro2::Span;
    use syn_solidity::Type;

    use crate::types::location::{Bound, Location};

    use super::*;
 
     #[test]
    fn new_good_construct() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
    
        assert_eq!(file.borrow().path, "File.test");
    }

    #[test]
    fn add_contract() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let contract = ContractReference::new("Contract".to_string(), Location::new("File.test".to_string(), Bound::new(0, 0), Bound::new(0, 0)), &file);
        file.borrow_mut().add_contract(contract);

        assert_eq!(file.borrow().contracts.len(), 1);
    }

    #[test]
    fn add_struct() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let strct = StructReference::new("Struct".to_string(), Location::new("File.test".to_string(), Bound::new(0, 0), Bound::new(0, 0)), None, Some(&file));
        file.borrow_mut().add_struct(strct);

        assert_eq!(file.borrow().structs.len(), 1);
    }

    #[test]
    fn add_enum() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let enm = EnumReference::new("Enum".to_string(), Location::new("File.test".to_string(), Bound::new(0, 0), Bound::new(0, 0)), None, Some(&file));
        file.borrow_mut().add_enum(enm);

        assert_eq!(file.borrow().enums.len(), 1);
    }

    #[test]
    fn add_error() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let error = ErrorReference::new("Error".to_string(), Location::new("File.test".to_string(), Bound::new(0, 0), Bound::new(0, 0)), None, None);
        file.borrow_mut().add_error(error);

        assert_eq!(file.borrow().errors.len(), 1);
    }

    #[test]
    fn add_event() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let event = EventReference::new("Event".to_string(), Location::new("File.test".to_string(), Bound::new(0, 0), Bound::new(0, 0)), None, None);
        file.borrow_mut().add_event(event);

        assert_eq!(file.borrow().events.len(), 1);
    }

    #[test]
    fn add_variable() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let variable = VariableReference::new("Variable".to_string(), Type::Bool(Span::call_site()), Location::new("File.test".to_string(), Bound::new(0, 0), Bound::new(0, 0)), None, None);
        file.borrow_mut().add_variable(variable);

        assert_eq!(file.borrow().variables.len(), 1);
    }
}