#[derive(Clone)]
pub struct Order {
    pub id: u64,
    pub price: u64,
}

// Only for buy side
pub trait OrderBook: IntoIterator<Item = Order> {
    fn new() -> Self;

    fn insert_order(&mut self, order: Order);

    /// To simulate matching
    fn pop_front(&mut self);
}
