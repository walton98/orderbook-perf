use std::collections::VecDeque;

use crate::order_book::{Order, OrderBook};

fn price_is_longer_or_equal(price: u64, other: &Order) -> bool {
    price <= other.price
}

// Orders are forwards
pub struct VecDequeBook {
    orders: VecDeque<Order>,
}

impl OrderBook for VecDequeBook {
    fn new() -> Self {
        let orders = VecDeque::new();
        Self { orders }
    }

    fn insert_order(&mut self, order: Order) {
        let predicate = |other: &_| price_is_longer_or_equal(order.price, other);
        let new_pos = self.orders.partition_point(predicate);
        self.orders.insert(new_pos, order);
    }

    fn pop_front(&mut self) {
        self.orders.remove(0);
    }
}

impl IntoIterator for VecDequeBook {
    type Item = Order;
    type IntoIter = <VecDeque<Order> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.orders.into_iter()
    }
}
