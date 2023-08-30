use std::cell::RefCell;
/**
 * EnumReference.rs
 * Definition of EnumReference struct
 * author: 0xMemoryGrinder
*/

use std::fmt;
use std::rc::Rc;
use crate::types::location::Location;
use crate::types::file_reference::FileReference;
use crate::types::contract_reference::ContractReference;

/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct EnumValueReference {
    pub name: String,
    pub location: Location,
}

pub struct EnumReference {
    pub name: String,
    pub location: Location,
    pub values : Vec<EnumValueReference>,
    pub contract: Option<Rc<RefCell<ContractReference>>>,
    pub file: Option<Rc<RefCell<FileReference>>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

 impl EnumReference {
    pub fn new(name: String, location: Location, contract: Option<&Rc<RefCell<ContractReference>>>, file: Option<&Rc<RefCell<FileReference>>>) -> EnumReference {
        EnumReference {
            name: name,
            location: location,
            values: Vec::new(),
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

    pub fn add_value(&mut self, name: String, location: Location) {
        self.values.push(EnumValueReference {
            name: name,
            location: location,
        });
    }
 }

impl fmt::Display for EnumReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enum {} at {}", self.name, self.location)
    }
}

impl fmt::Debug for EnumReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enum {} at {}", self.name, self.location)
    }
}

impl PartialEq for EnumReference {
    fn eq(&self, other: &EnumReference) -> bool {
        self.name == other.name && self.location == other.location && self.values == other.values && self.file == other.file
    }
}

impl PartialEq for EnumValueReference {
    fn eq(&self, other: &EnumValueReference) -> bool {
        self.name == other.name && self.location == other.location
    }
}

/******************************************************************************
 *                                  Tests                                     *
 *****************************************************************************/

#[cfg(test)]
mod tests {
    use crate::types::location::Bound;

    use super::*;

    #[test]
    fn new_good_construct() {
        let result = EnumReference::new(
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
        assert!(result.values.is_empty());
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
        let result = EnumReference::new(
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
        assert!(result.values.is_empty());

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
        let result = EnumReference::new(
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
        assert!(result.values.is_empty());

        assert_eq!(result.file.as_ref().unwrap().borrow().path, "File.test");
    }

    #[test]
    fn add_value() {
        let mut result = EnumReference::new(
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
            Location::new(
                String::from("test.sol"),
                Bound::new(0, 0),
                Bound::new(0, 0),
            ),
        );

        assert_eq!(result.values.len(), 1);
        assert_eq!(result.values[0].name, "TestValue");
        assert_eq!(result.values[0].location.file, "test.sol");
        assert_eq!(result.values[0].location.start.line, 0);
        assert_eq!(result.values[0].location.start.column, 0);
        assert_eq!(result.values[0].location.end.line, 0);
        assert_eq!(result.values[0].location.end.column, 0);
    }
}