use std::cell::RefCell;
/**
 * StructReference.rs
 * Definition of StructReference struct
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

pub struct StructReference {
    pub name: String,
    pub location: Location,
    pub contract: Option<Rc<RefCell<ContractReference>>>,
    pub file: Option<Rc<RefCell<FileReference>>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

 impl StructReference {
    pub fn new(name: String, location: Location, contract: Option<&Rc<RefCell<ContractReference>>>, file: Option<&Rc<RefCell<FileReference>>>) -> StructReference {
        StructReference {
            name: name,
            location: location,
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
 }

impl fmt::Display for StructReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Struct {} at {}", self.name, self.location)
    }
}

impl fmt::Debug for StructReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Struct {} at {}", self.name, self.location)
    }
}

impl PartialEq for StructReference {
    fn eq(&self, other: &StructReference) -> bool {
        self.name == other.name && self.location == other.location && self.contract == other.contract && self.file == other.file
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
        let result = StructReference::new(
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
    }

    #[test]
    fn new_contract_struct() {
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
        let result = StructReference::new(
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
        let result = StructReference::new(
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
 
        assert_eq!(result.file.as_ref().unwrap().borrow().path, "File.test");
    }
}