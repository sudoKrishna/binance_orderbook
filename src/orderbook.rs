use crate::types::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};

pub struct OrderBook {
    pub bids: BTreeMap<Reverse<u32>, Vec<Order>>, // Bids are stored with Reverse to prioritize higher prices
    pub asks: BTreeMap<u32, Vec<Order>>, // Asks are stored with normal u32 prices
    pub orders_by_id: HashMap<u64, Order>, // Maps order ID to the Order
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders_by_id: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let level = match order.side {
            Side::Buy => self.bids.entry(Reverse(order.price)).or_default(), // Reverse for bids
            Side::Sell => self.asks.entry(order.price).or_default(), // Direct price for asks
        };
        level.push(order.clone());
        self.orders_by_id.insert(order.id, order);
    }

    pub fn cancel_order(&mut self, order_id: u64) -> bool {
        if let Some(order) = self.orders_by_id.remove(&order_id) {
            let level = match order.side {
                Side::Buy => self.bids.get_mut(&Reverse(order.price)),
                Side::Sell => self.asks.get_mut(&order.price),
            };

            if let Some(orders) = level {
                orders.retain(|o| o.id != order_id);
                if orders.is_empty() {
                    match order.side {
                        Side::Buy => self.bids.remove(&Reverse(order.price)),
                        Side::Sell => self.asks.remove(&order.price),
                    };
                }
                return true;
            }
        }
        false
    }

    pub fn best_bid(&self) -> Option<&Order> {
        self.bids.iter().next().and_then(|(_, orders)| orders.first())
    }

    pub fn best_ask(&self) -> Option<&Order> {
        self.asks.iter().next().and_then(|(_, orders)| orders.first())
    }

    pub fn match_order(&mut self, mut incoming: Order) {
        match incoming.side {
            Side::Buy => {
                let mut to_remove = vec![];

                // Iterate through asks (which are stored as BTreeMap<u32, Vec<Order>>)
                for (price, orders) in self.asks.iter_mut() {
                    if incoming.price < *price {
                        break; // No more matches possible
                    }

                    let mut i = 0;
                    while i < orders.len() && incoming.quantity > 0 {
                        let book_order = &mut orders[i];
                        let traded_qty = incoming.quantity.min(book_order.quantity);

                        println!(
                            "Matched: incoming #{:?} <=> book #{:?} @ {} for {}",
                            incoming.id, book_order.id, price, traded_qty
                        );

                        incoming.quantity -= traded_qty;
                        book_order.quantity -= traded_qty;

                        if book_order.quantity == 0 {
                            i += 1;
                        }
                    }

                    // Clean up fully matched orders
                    orders.retain(|o| o.quantity > 0);
                    if orders.is_empty() {
                        to_remove.push(*price); // Collect prices to remove
                    }

                    if incoming.quantity == 0 {
                        break; // Stop if order is fully matched
                    }
                }

                // Remove empty levels
                for price in to_remove {
                    self.asks.remove(&price);
                }

                if incoming.quantity > 0 {
                    self.add_order(incoming); // Add the remaining order to the order book
                }
            }

            Side::Sell => {
                let mut to_remove = vec![];

                // Iterate through bids (which are stored as BTreeMap<Reverse<u32>, Vec<Order>>)
                for (price, orders) in self.bids.iter_mut() {
                    let price = price.0;  // Unwrap Reverse here
                    if incoming.price > price {
                        break; // No more matches possible
                    }

                    let mut i = 0;
                    while i < orders.len() && incoming.quantity > 0 {
                        let book_order = &mut orders[i];
                        let traded_qty = incoming.quantity.min(book_order.quantity);

                        println!(
                            "Matched: incoming #{:?} <=> book #{:?} @ {} for {}",
                            incoming.id, book_order.id, price, traded_qty
                        );

                        incoming.quantity -= traded_qty;
                        book_order.quantity -= traded_qty;

                        if book_order.quantity == 0 {
                            i += 1;
                        }
                    }

                    // Clean up fully matched orders
                    orders.retain(|o| o.quantity > 0);
                    if orders.is_empty() {
                        to_remove.push(Reverse(price)); // Collect Reverse-wrapped prices to remove
                    }

                    if incoming.quantity == 0 {
                        break; // Stop if order is fully matched
                    }
                }

                // Remove empty levels
                for price in to_remove {
                    self.bids.remove(&price);
                }

                if incoming.quantity > 0 {
                    self.add_order(incoming); // Add the remaining order to the order book
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Order, Side};

    #[test]
    fn test_add_order() {
        let mut ob = OrderBook::new();
        ob.add_order(Order::new(1, 100, 10, Side::Buy));
        assert_eq!(ob.orders_by_id.len(), 1);
        assert_eq!(ob.best_bid().unwrap().price, 100);
    }

    #[test]
    fn test_cancel_order() {
        let mut ob = OrderBook::new();
        ob.add_order(Order::new(1, 100, 10, Side::Buy));
        assert!(ob.cancel_order(1));
        assert!(ob.best_bid().is_none());
    }

    #[test]
    fn test_match_order_buy() {
        let mut ob = OrderBook::new();
        ob.add_order(Order::new(1, 100, 10, Side::Sell));
        ob.match_order(Order::new(2, 100, 4, Side::Buy));
        assert_eq!(ob.best_ask().unwrap().quantity, 6);
    }

    #[test]
    fn test_match_order_sell_partial() {
        let mut ob = OrderBook::new();
        ob.add_order(Order::new(1, 100, 8, Side::Buy));
        ob.match_order(Order::new(2, 100, 5, Side::Sell));
        assert_eq!(ob.best_bid().unwrap().quantity, 3);
    }

    #[test]
    fn test_best_bid_ask() {
        let mut ob = OrderBook::new();
        ob.add_order(Order::new(1, 101, 5, Side::Buy));
        ob.add_order(Order::new(2, 102, 5, Side::Buy));
        ob.add_order(Order::new(3, 99, 5, Side::Sell));
        assert_eq!(ob.best_bid().unwrap().price, 102);
        assert_eq!(ob.best_ask().unwrap().price, 99);
    }
}
