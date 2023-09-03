/**
 * VariableReference.rs
 * Definition of VariableReference struct
 * author: 0xMemoryGrinder
 */

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use syn_solidity::Type;

use crate::types::location::Location;
use crate::types::contract_reference::ContractReference;

use super::file_reference::FileReference;


/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct VariableReference {
    pub name: String,
    pub ty: Type,
    pub location: Location,
    pub contract: Option<Rc<RefCell<ContractReference>>>,
    pub file: Option<Rc<RefCell<FileReference>>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

 impl VariableReference {
    pub fn new(name: String, ty: Type, location: Location, contract: Option<&Rc<RefCell<ContractReference>>>, file: Option<&Rc<RefCell<FileReference>>>) -> VariableReference {
        VariableReference {
            name: name,
            ty,
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

impl fmt::Display for VariableReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {} at {}", self.name, self.location)
    }
}

impl fmt::Debug for VariableReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {} at {} in contract {:?}", self.name, self.location, self.contract)
    }
}

impl PartialEq for VariableReference {
    fn eq(&self, other: &VariableReference) -> bool {
        self.name == other.name && self.location == other.location && self.contract == other.contract
    }
}

/******************************************************************************
 *                                  Tests                                     *
 *****************************************************************************/

 #[cfg(test)]
 mod tests {
    use std::cell::RefCell; 
    use proc_macro2::Span;

    use crate::types::{location::{Bound, Location}, file_reference::FileReference};

    use super::*;
 
     #[test]
    fn new_good_construct() {
        let file = Rc::new(RefCell::new(FileReference::new("test.sol".to_string())));
        let contract = Rc::new(RefCell::new(ContractReference::new("Test".to_string(), Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)), &file)));
        let property = VariableReference::new("test".to_string(), Type::Bool(Span::call_site()), Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)), Some(&contract), None);
        assert_eq!(property.name, "test");
        assert_eq!(property.ty, Type::Bool(Span::call_site()));
        assert_eq!(property.location, Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)));
        assert_eq!(property.contract, Some(contract));
        assert_eq!(property.file, None);
    }

    #[test]
    fn new_standalone_variable() {
        let file = Rc::new(RefCell::new(FileReference::new("test.sol".to_string())));
        let property = VariableReference::new("test".to_string(), Type::Bool(Span::call_site()), Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)), None, Some(&file));
        assert_eq!(property.name, "test");
        assert_eq!(property.ty, Type::Bool(Span::call_site()));
        assert_eq!(property.location, Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)));
        assert_eq!(property.contract, None);
        assert_eq!(property.file, Some(file));
    }
}