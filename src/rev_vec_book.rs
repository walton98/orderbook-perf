use crate::order_book::{Order, OrderBook};
use std::iter::Rev;

fn price_is_shorter(price: u64, other: &Order) -> bool {
    price > other.price
}

// Orders are backwards
pub struct RevVecBook {
    orders: Vec<Order>,
}

impl OrderBook for RevVecBook {
    fn new() -> Self {
        let orders = Vec::new();
        Self { orders }
    }

    fn insert_order(&mut self, order: Order) {
        let predicate = |other: &_| price_is_shorter(order.price, other);
        let new_pos = self.orders.partition_point(predicate);
        self.orders.insert(new_pos, order);
    }

    fn pop_front(&mut self) {
        self.orders.pop().unwrap();
    }
}

impl IntoIterator for RevVecBook {
    type Item = Order;
    type IntoIter = Rev<<Vec<Order> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.orders.into_iter().rev()
    }
}
