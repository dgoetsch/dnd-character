use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod check;
pub mod damage;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Dice {
    count: isize,
    sides: isize,
}

impl Dice {
    pub fn new(count: isize, sides: isize) -> Dice {
        Dice { count, sides }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)
    }
}
