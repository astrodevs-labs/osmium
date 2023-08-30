/**
 * ContractReference.rs
 * Definition of ContractReference struct
 * author: 0xMemoryGrinder
*/

use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use crate::types::location::Location;
use crate::types::file_reference::FileReference;
use crate::types::struct_reference::StructReference;
use crate::types::function_reference::FunctionReference;
use crate::types::property_reference::PropertyReference;

/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct ContractReference {
    pub name: String,
    pub location: Location,
    pub file: Rc<RefCell<FileReference>>,
    pub structs: Vec<Rc<RefCell<StructReference>>>,
    pub functions: Vec<Rc<RefCell<FunctionReference>>>,
    pub properties: Vec<Rc<RefCell<PropertyReference>>>,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

impl ContractReference {
    pub fn new(name: String, location: Location, file: &Rc<RefCell<FileReference>>) -> ContractReference {
        ContractReference {
            name: name,
            location: location,
            file: file.clone(),
            structs: Vec::new(),
            functions: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn add_struct(&mut self, strct: &Rc<RefCell<StructReference>>) {
        self.structs.push(strct.clone());
    }

    pub fn add_function(&mut self, function: &Rc<RefCell<FunctionReference>>) {
        self.functions.push(function.clone());
    }

    pub fn add_property(&mut self, property: &Rc<RefCell<PropertyReference>>) {
        self.properties.push(property.clone());
    }
}

impl fmt::Display for ContractReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contract {} at {}", self.name, self.location)
    }
}

impl fmt::Debug for ContractReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contract {} at {}", self.name, self.location)
    }
}

impl PartialEq for ContractReference {
    fn eq(&self, other: &ContractReference) -> bool {
        self.name == other.name && self.location == other.location && self.file == other.file
    }
}

impl Eq for ContractReference {}

impl Hash for ContractReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.location.hash(state);
    }
}

/******************************************************************************
 *                                  Tests                                     *
 *****************************************************************************/

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::types::location::Bound;

    use super::*;

    #[test]
    fn new_good_construct() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let result = ContractReference::new("Test".to_string(), Location::new("File.test".to_string(), Bound {line: 0, column: 0}, Bound { line: 0, column: 0}), &file);

        assert_eq!(result.file, file);
        assert_eq!(result.name, "Test");
        assert_eq!(result.location.file, "File.test");
        assert_eq!(result.location.start.line, 0);
        assert_eq!(result.location.start.column, 0);
        assert_eq!(result.location.end.line, 0);
        assert_eq!(result.location.end.column, 0);
    }

    #[test]
    fn add_struct() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let result = Rc::new(RefCell::new(ContractReference::new("Test".to_string(), Location::new("File.test".to_string(), Bound {line: 0, column: 0}, Bound { line: 0, column: 0}), &file)));
        let strct = Rc::new(RefCell::new(StructReference::new("TestStruct".to_string(), Location::new("File.test".to_string(), Bound {line: 0, column: 0}, Bound { line: 0, column: 0}), Some(&result), Some(&file))));

        (*result).borrow_mut().add_struct(&strct);

        assert_eq!(result.borrow().structs.len(), 1);
        assert_eq!(result.borrow().structs[0], strct);
    }
    
    #[test]
    fn add_function() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let result = Rc::new(RefCell::new(ContractReference::new("Test".to_string(), Location::new("File.test".to_string(), Bound {line: 0, column: 0}, Bound { line: 0, column: 0}), &file)));
        let function = Rc::new(RefCell::new(FunctionReference::new("TestFunction".to_string(), Location::new("File.test".to_string(), Bound {line: 0, column: 0}, Bound { line: 0, column: 0}), &result)));

        (*result).borrow_mut().add_function(&function);

        assert_eq!(result.borrow().functions.len(), 1);
        assert_eq!(result.borrow().functions[0], function);
    }

    #[test]
    fn add_property() {
        let file = Rc::new(RefCell::new(FileReference::new("File.test".to_string())));
        let result = Rc::new(RefCell::new(ContractReference::new("Test".to_string(), Location::new("File.test".to_string(), Bound {line: 0, column: 0}, Bound { line: 0, column: 0}), &file)));
        let property = Rc::new(RefCell::new(PropertyReference::new("TestProperty".to_string(), "uint256".to_string(), Location::new("File.test".to_string(), Bound {line: 0, column: 0}, Bound { line: 0, column: 0}), &result)));

        (*result).borrow_mut().add_property(&property);

        assert_eq!(result.borrow().properties.len(), 1);
        assert_eq!(result.borrow().properties[0], property);
    }
}