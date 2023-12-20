mod dummy_book;
mod order_gen;

use criterion::{criterion_group, criterion_main, Criterion};

use order_book::{ListBook, Order, OrderBook, RevVecBook, VecBook, VecDequeBook};

use crate::dummy_book::DummyBook;
use crate::order_gen::OrderGen;

fn insert_orders(book: &mut impl OrderBook, orders: impl Iterator<Item = Order>) {
    for order in orders {
        book.insert_order(order);
    }
}

fn pop_orders(book: &mut impl OrderBook, num_orders: usize) {
    for _ in 0..num_orders {
        book.pop_front();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut order_gen = OrderGen::new();

    macro_rules! book_tests {
        ($($name:ident: $type:ty,)*) => {
        $(
            c.bench_function(&format!("{}: insert", stringify!($name)), |b| b.iter(|| {
                insert_orders(&mut <$type>::new(), (&mut order_gen).take(1000));
            }));
            c.bench_function(&format!("{}: insert and match", stringify!($name)), |b| b.iter(|| {
                let num_orders = 1000;
                let mut book = <$type>::new();
                insert_orders(&mut book, (&mut order_gen).take(num_orders));
                pop_orders(&mut book, num_orders);
            }));
        )*
        }
    }
    book_tests! {
        vec: VecBook,
        vec_deque: VecDequeBook,
        rev_vec: RevVecBook,
        list_book: ListBook,
        dummy: DummyBook,
    };
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(250);
    targets = criterion_benchmark
}
criterion_main!(benches);
