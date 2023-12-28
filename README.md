# Order Book Benchmarks

[![Order Book CI](https://github.com/walton98/orderbook-perf/actions/workflows/ci.yml/badge.svg)](https://github.com/walton98/orderbook-perf/actions/workflows/ci.yml)

```
$ cargo +nightly bench


vec: insert             time:   [64.336 µs 64.566 µs 64.832 µs]

vec: insert and match   time:   [127.80 µs 130.58 µs 133.33 µs]

vec_deque: insert       time:   [66.005 µs 67.547 µs 69.317 µs]

vec_deque: insert and match
                        time:   [64.830 µs 65.818 µs 66.852 µs]

rev_vec: insert         time:   [71.560 µs 72.318 µs 73.183 µs]

rev_vec: insert and match
                        time:   [73.253 µs 74.227 µs 75.313 µs]

list_book: insert       time:   [340.72 µs 345.78 µs 351.76 µs]

list_book: insert and match
                        time:   [349.62 µs 356.14 µs 364.74 µs]

dummy: insert           time:   [3.8012 µs 3.9063 µs 4.0258 µs]

dummy: insert and match time:   [3.6138 µs 3.6618 µs 3.7204 µs]
```
