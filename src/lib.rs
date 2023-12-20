pub mod order_book;
pub mod rev_vec_book;
pub mod vec_book;
pub mod vec_deque_book;

pub use crate::order_book::{Order, OrderBook};
pub use crate::rev_vec_book::RevVecBook;
pub use crate::vec_book::VecBook;
pub use crate::vec_deque_book::VecDequeBook;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! book_tests {
        ($($name:ident: $type:ty,)*) => {
        $(
            mod $name {
                use super::*;

                #[test]
                fn insertion_same_price() {
                    let mut book = <$type>::new();
                    for i in 1..10 {
                        let order = Order{
                            id: i,
                            price: 1,
                        };

                        book.insert_order(order);
                    }

                    let expected: Vec<_> = (1..10).collect();
                    let orders: Vec<_> = book.into_iter().map(|o| o.id).collect();
                    assert_eq!(expected, orders);
                }

                #[test]
                fn insertion_different_price() {
                    let mut book = VecBook::new();
                    for i in 1..10 {
                        let order = Order{
                            id: i,
                            price: i,
                        };

                        book.insert_order(order);
                    }

                    let expected: Vec<_> = (1..10).rev().collect();
                    let orders: Vec<_> = book.into_iter().map(|o| o.id).collect();
                    assert_eq!(expected, orders);
                }

            }
        )*
        }
    }

    book_tests! {
        vec_book: VecBook,
        vec_deque_book: VecDequeBook,
        rev_vec_book: RevVecBook,
    }
}
