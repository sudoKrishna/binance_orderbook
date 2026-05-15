use std::collections::{BTreeMap, HashMap};
use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub price: u32,      
    pub quantity: u32,   
    pub side: Side,
}

pub type PriceLevel = Vec<Order>;
pub type Bids = BTreeMap<Reverse<u32>, PriceLevel>; 
pub type Asks = BTreeMap<u32, PriceLevel>;          
pub type OrderMap = HashMap<u64, Order>;
