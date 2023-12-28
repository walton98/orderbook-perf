use order_book::Order;
use rand::distributions::{Distribution, Uniform};

pub struct OrderGen {
    id_seq: u64,
    rng: rand::rngs::ThreadRng,
    price_dist: Uniform<u64>,
}

impl OrderGen {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let price_dist = Uniform::from(10..30);
        Self {
            id_seq: 0,
            rng,
            price_dist,
        }
    }
}

impl Iterator for OrderGen {
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        let order = Order {
            id: self.id_seq,
            price: self.price_dist.sample(&mut self.rng),
        };
        self.id_seq += 1;
        Some(order)
    }
}
