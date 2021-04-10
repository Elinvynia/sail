use crate::components::{Money, Player};
use hecs::Bundle;

pub fn player() -> impl Bundle {
    (Money::new(100), Player)
}
