use broken_app::{algo, concurrency, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sums_even_empty() {
    assert_eq!(sum_even(&[]), 0);
}

#[test]
fn sums_even_all_odd() {
    assert_eq!(sum_even(&[1, 3, 5]), 0);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn counts_non_zero_empty() {
    assert_eq!(leak_buffer(&[]), 0);
}

#[test]
fn counts_non_zero_all_zeros() {
    assert_eq!(leak_buffer(&[0, 0, 0]), 0);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]);
}

#[test]
fn dedup_empty() {
    let uniq = algo::slow_dedup(&[]);
    assert_eq!(uniq, vec![]);
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn fib_zero() {
    assert_eq!(algo::slow_fib(0), 0);
}

#[test]
fn fib_one() {
    assert_eq!(algo::slow_fib(1), 1);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_tabs() {
    assert_eq!(normalize("Hello\tWorld"), "helloworld");
}

#[test]
fn normalize_multiple_spaces() {
    assert_eq!(normalize("  a   b  "), "ab");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn averages_empty() {
    assert!((broken_app::average_positive(&[]) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn averages_all_negative() {
    assert!((broken_app::average_positive(&[-1, -2, -3]) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn race_increment_tests() {
    // Run sequentially in one test to avoid shared global state conflicts
    concurrency::reset_counter();
    let total = concurrency::race_increment(1_000, 4);
    assert_eq!(total, 4_000);

    concurrency::reset_counter();
    let total = concurrency::race_increment(100, 1);
    assert_eq!(total, 100);
}
