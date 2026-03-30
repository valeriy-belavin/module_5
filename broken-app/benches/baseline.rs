use broken_app::{algo, sum_even};
use std::time::Instant;

fn time_it(label: &str, mut f: impl FnMut()) {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("{label}: {:?}", elapsed);
}

fn main() {
    let data: Vec<i64> = (0..50_000).collect();
    let fib_n = 32;
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();

    for _ in 0..3 {
        time_it("sum_even", || {
            std::hint::black_box(sum_even(std::hint::black_box(&data)));
        });

        time_it("slow_fib", || {
            std::hint::black_box(algo::slow_fib(std::hint::black_box(fib_n)));
        });

        time_it("slow_dedup", || {
            std::hint::black_box(algo::slow_dedup(std::hint::black_box(&dedup_data)));
        });
    }
}
