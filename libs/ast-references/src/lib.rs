mod types;
use std::cell::RefCell;
use std::rc::Rc;

use types::contract_reference::ContractReference;
use types::location::{Location, Bound};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn parse () -> ContractReference {
    let file = Rc::new(RefCell::new(types::file_reference::FileReference::new("".to_string())));
    ContractReference::new("".to_string(), Location::new("".to_string(), Bound { line: 0, column: 0}, Bound { line: 0, column: 0}), &file)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
