mod dummy_book;
mod order_gen;

use std::any::type_name;

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

fn benchmark<B: OrderBook>(c: &mut Criterion) {
    let mut order_gen = OrderGen::new();

    let mut group = c.benchmark_group(type_name::<B>());

    group.bench_function(&format!("insert"), |b| {
        b.iter(|| {
            insert_orders(&mut B::new(), (&mut order_gen).take(1000));
        })
    });

    group.bench_function(&format!("insert and match"), |b| {
        b.iter(|| {
            let num_orders = 1000;
            let mut book = B::new();
            insert_orders(&mut book, (&mut order_gen).take(num_orders));
            pop_orders(&mut book, num_orders);
        })
    });
}

criterion_group!(vec_book, benchmark::<VecBook>);
criterion_group!(vec_deque_book, benchmark::<VecDequeBook>);
criterion_group!(rev_vec_book, benchmark::<RevVecBook>);
criterion_group!(list_book, benchmark::<ListBook>);
criterion_group!(dummy_book, benchmark::<DummyBook>);
criterion_main!(vec_book, vec_deque_book);
