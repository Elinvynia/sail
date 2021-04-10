#[derive(Debug)]
pub struct Money {
    pub amount: u64,
}

impl Money {
    pub fn new(amount: u64) -> Self {
        Money { amount }
    }
}
