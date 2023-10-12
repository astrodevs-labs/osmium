/**
 * Location.rs
 * Definition of Location and Bound structs
 * author: 0xMemoryGrinder
*/

use std::fmt;
use std::hash::{Hash, Hasher};

/******************************************************************************
 *                                  Types                                     *
 *****************************************************************************/

pub struct Bound {
    pub line: u32,
    pub column: u32,
}

pub struct Location {
    pub file: String, // path of the file
    pub start: Bound,
    pub end: Bound,
}

/******************************************************************************
 *                        Methods / Trait implementation                      *
 *****************************************************************************/

impl Bound {
    pub fn new(line: u32, column: u32) -> Bound {
        Bound {
            line: line,
            column: column,
        }
    }
}

impl Location {
    pub fn new(file: String, start: Bound, end: Bound) -> Location {
        Location {
            file: file,
            start: start,
            end: end,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:({}:{}-{}:{})", self.file, self.start.line, self.start.column, self.end.line, self.end.column)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:({}:{}-{}:{})", self.file, self.start.line, self.start.column, self.end.line, self.end.column)
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.file == other.file && self.start == other.start && self.end == other.end
    }
}

impl PartialEq for Bound {
    fn eq(&self, other: &Bound) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.start.hash(state);
        self.end.hash(state);
 
   }
}

impl Hash for Bound {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.line.hash(state);
        self.column.hash(state);
    }
}

impl Clone for Location {
    fn clone(&self) -> Location {
        Location {
            file: self.file.clone(),
            start: Bound {
                line: self.start.line,
                column: self.start.column,
            },
            end: Bound {
                line: self.end.line,
                column: self.end.column,
            },
        }
    }
}

impl Default for Location {
    fn default() -> Location {
        Location {
            file: String::new(),
            start: Bound {
                line: 0,
                column: 0,
            },
            end: Bound {
                line: 0,
                column: 0,
            },
        }
    }
}

/******************************************************************************
 *                                  Tests                                     *
 *****************************************************************************/

 #[cfg(test)]
 mod tests {
    use super::*;
 
     #[test]
    fn new_good_construct() {
        let result = Location::new(
            String::from("test.sol"),
            Bound::new(0, 0),
            Bound::new(0, 0),
        );

        assert_eq!(result.file, "test.sol");
        assert_eq!(result.start.line, 0);
        assert_eq!(result.start.column, 0);
        assert_eq!(result.end.line, 0);
        assert_eq!(result.end.column, 0);
    }
}