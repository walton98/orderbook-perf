use criterion::black_box;
use order_book::{Order, OrderBook};
use std::iter;

pub struct DummyBook {}

impl OrderBook for DummyBook {
    fn new() -> Self {
        Self {}
    }

    fn insert_order(&mut self, order: Order) {
        black_box(order);
    }

    fn pop_front(&mut self) {}
}

impl IntoIterator for DummyBook {
    type Item = Order;
    type IntoIter = <iter::Empty<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}
