mod order;
mod orderbook;
mod types;

use orderbook::OrderBook;
use types::{Order, Side};

fn main() {
    let mut ob = OrderBook::new();

    ob.add_order(Order::new(1, 100, 5, Side::Sell));
    ob.add_order(Order::new(2, 99, 3, Side::Sell));
    ob.add_order(Order::new(3, 101, 2, Side::Buy));

    println!("Best Ask: {:?}", ob.best_ask());
    println!("Best Bid: {:?}", ob.best_bid());

    ob.match_order(Order::new(4, 100, 4, Side::Buy));

    println!("After matching:");
    println!("Best Ask: {:?}", ob.best_ask());
    println!("Best Bid: {:?}", ob.best_bid());
}
