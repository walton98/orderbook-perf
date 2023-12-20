use crate::order_book::{Order, OrderBook};

fn price_is_longer_or_equal(price: u64, other: &Order) -> bool {
    price <= other.price
}

// Orders are forwards
pub struct VecBook {
    orders: Vec<Order>,
}

impl OrderBook for VecBook {
    fn new() -> Self {
        let orders = Vec::new();
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

impl IntoIterator for VecBook {
    type Item = Order;
    type IntoIter = <Vec<Order> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.orders.into_iter()
    }
}
