use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub name: ItemName,
    pub amount: u64,
}

impl Item {
    pub fn new(name: ItemName, amount: u64) -> Self {
        Item { name, amount }
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new(items: Vec<Item>) -> Self {
        Inventory { items }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemName {
    Rum,
    Bananas,
    Water,
}

impl fmt::Display for ItemName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ItemName::*;
        let s = match self {
            Rum => "Rum",
            Bananas => "Bananas",
            Water => "Water",
        };

        write!(f, "{}", s)
    }
}
