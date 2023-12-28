use std::collections::LinkedList;

use crate::order_book::{Order, OrderBook};

fn price_is_longer_or_equal(price: u64, other: &Order) -> bool {
    price <= other.price
}

// Orders are forwards
pub struct ListBook {
    orders: LinkedList<Order>,
}

impl OrderBook for ListBook {
    fn new() -> Self {
        let orders = LinkedList::new();
        Self { orders }
    }

    fn insert_order(&mut self, order: Order) {
        let mut cursor = self.orders.cursor_front_mut();
        while let Some(value) = cursor.current() {
            if price_is_longer_or_equal(order.price, value) {
                cursor.move_next();
            } else {
                break;
            }
        }
        cursor.insert_before(order);
    }

    fn remove_order(&mut self, _order_id: u64) {
        // TODO
    }

    fn pop_front(&mut self) {
        self.orders.pop_front();
    }
}

impl IntoIterator for ListBook {
    type Item = Order;
    type IntoIter = <LinkedList<Order> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.orders.into_iter()
    }
}
