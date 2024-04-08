use core::fmt;

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.id, self.name)
    }
}
