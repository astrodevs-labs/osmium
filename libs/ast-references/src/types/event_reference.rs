/**
 * EventReference.rs
 * Definition of EventReference struct
 * author: 0xMemoryGrinder
 */

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use ast_extractor::{Type, Storage};
use crate::types::location::Location;
use crate::types::file_reference::FileReference;
use crate::types::contract_reference::ContractReference;

/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct EventParameterReference {
    pub name: String,
    pub ty: Type,
    pub storage: Option<Storage>,
    pub location: Location,
}

pub struct EventReference {
    pub name: String,
    pub location: Location,
    pub parameters : Vec<EventParameterReference>,
    pub contract: Option<Rc<RefCell<ContractReference>>>,
    pub file: Option<Rc<RefCell<FileReference>>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

 impl EventReference {
    pub fn new(name: String, location: Location, contract: Option<&Rc<RefCell<ContractReference>>>, file: Option<&Rc<RefCell<FileReference>>>) -> EventReference {
        EventReference {
            name: name,
            location: location,
            parameters: Vec::new(),
            contract: match contract {
                Some(c) => Some(c.clone()),
                None => None,
            },
            file: match file {
                Some(f) => Some(f.clone()),
                None => None,
            },
        }
    }

    pub fn add_value(&mut self, name: String, ty: Type, storage: Option<Storage>, location: Location) {
        self.parameters.push(EventParameterReference {
            name: name,
            ty: ty,
            storage: storage,
            location: location,
        });
    }
 }

impl fmt::Display for EventReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event {} at {}", self.name, self.location)
    }
}

impl fmt::Debug for EventReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event {} at {}", self.name, self.location)
    }
}

impl PartialEq for EventReference {
    fn eq(&self, other: &EventReference) -> bool {
        self.name == other.name && self.location == other.location && self.parameters == other.parameters && self.file == other.file
    }
}

impl PartialEq for EventParameterReference {
    fn eq(&self, other: &EventParameterReference) -> bool {
        self.name == other.name && self.location == other.location && self.ty == other.ty && self.storage == other.storage
    }
}

/******************************************************************************
 *                                  Tests                                     *
 *****************************************************************************/

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;
    use proc_macro2::Span;

    use crate::types::location::Bound;

    use super::*;

    #[test]
    fn new_good_construct() {
        let result = EventReference::new(
            String::from("Test"),
            Location::new(
                String::from("test.sol"),
                Bound::new(0, 0),
                Bound::new(0, 0),
            ),
            None,
            None,
        );

        assert_eq!(result.name, "Test");
        assert_eq!(result.location.file, "test.sol");
        assert_eq!(result.location.start.line, 0);
        assert_eq!(result.location.start.column, 0);
        assert_eq!(result.location.end.line, 0);
        assert_eq!(result.location.end.column, 0);
        assert!(result.parameters.is_empty());
    }

    #[test]
    fn new_contract_enum() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let contract = ContractReference::new(
            "Contract".to_string(),
            Location::new(
                "File.test".to_string(),
                Bound::new(0, 0),
                Bound::new(0, 0),
            ),
            &file,
        );
        let result = EventReference::new(
            String::from("Test"),
            Location::new(
                String::from("test.sol"),
                Bound::new(0, 0),
                Bound::new(0, 0),
            ),
            Some(&Rc::new(RefCell::new(contract))),
            None,
        );

        assert_eq!(result.name, "Test");
        assert_eq!(result.location.file, "test.sol");
        assert_eq!(result.location.start.line, 0);
        assert_eq!(result.location.start.column, 0);
        assert_eq!(result.location.end.line, 0);
        assert_eq!(result.location.end.column, 0);
        assert!(result.parameters.is_empty());

        assert_eq!(result.contract.as_ref().unwrap().borrow().name, "Contract");
        assert_eq!(result.contract.as_ref().unwrap().borrow().location.file, "File.test".to_string());
        assert_eq!(result.contract.as_ref().unwrap().borrow().location.start.line, 0);
        assert_eq!(result.contract.as_ref().unwrap().borrow().location.start.column, 0);
        assert_eq!(result.contract.as_ref().unwrap().borrow().location.end.line, 0);
        assert_eq!(result.contract.as_ref().unwrap().borrow().location.end.column, 0);
    }

    #[test]
    fn new_standalone_enum() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let result = EventReference::new(
            String::from("Test"),
            Location::new(
                String::from("test.sol"),
                Bound::new(0, 0),
                Bound::new(0, 0),
            ),
            None,
            Some(&file),
        );

        assert_eq!(result.name, "Test");
        assert_eq!(result.location.file, "test.sol");
        assert_eq!(result.location.start.line, 0);
        assert_eq!(result.location.start.column, 0);
        assert_eq!(result.location.end.line, 0);
        assert_eq!(result.location.end.column, 0);
        assert!(result.parameters.is_empty());

        assert_eq!(result.file.as_ref().unwrap().borrow().path, "File.test");
    }

    #[test]
    fn add_value() {
        let mut result = EventReference::new(
            String::from("Test"),
            Location::new(
                String::from("test.sol"),
                Bound::new(0, 0),
                Bound::new(0, 0),
            ),
            None,
            None,
        );

        result.add_value(
            String::from("TestValue"),
            Type::Uint(Span::call_site(), NonZeroU16::new(256)),
            None, 
            Location::new(
                String::from("test.sol"),
                Bound::new(0, 0),
                Bound::new(0, 0),
            ),
        );

        assert_eq!(result.parameters.len(), 1);
        assert_eq!(result.parameters[0].name, "TestValue");
        assert_eq!(result.parameters[0].location.file, "test.sol");
        assert_eq!(result.parameters[0].location.start.line, 0);
        assert_eq!(result.parameters[0].location.start.column, 0);
        assert_eq!(result.parameters[0].location.end.line, 0);
        assert_eq!(result.parameters[0].location.end.column, 0);
    }
}