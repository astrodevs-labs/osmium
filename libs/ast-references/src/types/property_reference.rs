use std::cell::RefCell;
/**
 * PropertyReference.rs
 * Definition of PropertyReference struct
 * author: 0xMemoryGrinder
*/

use std::fmt;
use std::rc::Rc;
use crate::types::location::Location;
use crate::types::contract_reference::ContractReference;


/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct PropertyReference {
    pub name: String,
    pub prop_type: String,
    pub location: Location,
    pub contract: Rc<RefCell<ContractReference>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

 impl PropertyReference {
    pub fn new(name: String, prop_type: String, location: Location, contract: &Rc<RefCell<ContractReference>>) -> PropertyReference {
        PropertyReference {
            name: name,
            prop_type,
            location: location,
            contract: contract.clone(),
        }
    }
 }

impl fmt::Display for PropertyReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {} at {}", self.name, self.location)
    }
}

impl fmt::Debug for PropertyReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {} at {} in contract {:?}", self.name, self.location, self.contract)
    }
}

impl PartialEq for PropertyReference {
    fn eq(&self, other: &PropertyReference) -> bool {
        self.name == other.name && self.location == other.location && self.contract == other.contract
    }
}

/******************************************************************************
 *                                  Tests                                     *
 *****************************************************************************/

 #[cfg(test)]
 mod tests {
    use std::cell::RefCell; 
    use crate::types::{location::{Bound, Location}, file_reference::FileReference};

    use super::*;
 
     #[test]
    fn new_good_construct() {
        let file = Rc::new(RefCell::new(FileReference::new("test.sol".to_string())));
        let contract = Rc::new(RefCell::new(ContractReference::new("Test".to_string(), Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)), &file)));
        let property = PropertyReference::new("test".to_string(), "uint256".to_string(), Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)), &contract);
        assert_eq!(property.name, "test");
        assert_eq!(property.prop_type, "uint256");
        assert_eq!(property.location, Location::new("test.sol".to_string(), Bound::new(0, 0), Bound::new(0, 0)));
        assert_eq!(property.contract, contract);
    }
}