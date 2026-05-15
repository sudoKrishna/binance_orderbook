use crate::types::{Order, Side};

impl Order {
    pub fn new(id: u64, price: u32, quantity: u32, side: Side) -> Self {
        Self {
            id,
            price,
            quantity,
            side,
        }
    }
}
