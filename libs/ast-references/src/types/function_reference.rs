use std::cell::RefCell;
/**
 * FunctionReference.rs
 * Definition of FuntionReference struct
 * author: 0xMemoryGrinder
*/

use std::fmt;
use std::rc::Rc;
use ast_extractor::FunctionKind;

use crate::types::location::Location;
use crate::types::contract_reference::ContractReference;

/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct FunctionReference {
    pub name: String,
    pub kind: FunctionKind,
    pub location: Location,
    pub contract: Rc<RefCell<ContractReference>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

 impl FunctionReference {
    pub fn new(name: String, kind: FunctionKind, location: Location, contract: &Rc<RefCell<ContractReference>>) -> FunctionReference {
        FunctionReference {
            name: name,
            kind: kind,
            location: location,
            contract: contract.clone(),
        }
    }
 }

impl fmt::Display for FunctionReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function {} at {}", self.name, self.location)
    }
}

impl fmt::Debug for FunctionReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function {} at {} in contract {:?}", self.name, self.location, self.contract)
    }
}

impl PartialEq for FunctionReference {
    fn eq(&self, other: &FunctionReference) -> bool {
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
    use ast_extractor::kw::function;

    use crate::types::{location::{Bound, Location}, file_reference::FileReference};

    use super::*;
 
     #[test]
    fn new_good_construct() {
        let file = Rc::new(RefCell::new(FileReference::new("Test.sol".to_string())));
        let contract = Rc::new(RefCell::new(ContractReference::new("contract".to_string(), Location::new("Test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)), &file)));
        let function = FunctionReference::new(
            "function".to_string(), 
            FunctionKind::Function(function(Span::call_site())),
            Location::new("Test.sol".to_string(), 
            Bound::new(0, 0), 
            Bound::new(0, 0)), 
            &contract
        );

        assert_eq!(function.name, "function".to_string());
        assert_eq!(function.location, Location::new("Test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)));
        assert_eq!(function.contract, contract);
    }
}