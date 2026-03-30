use std::collections::HashSet;

/// Алгоритмическая оптимизация: O(n log n) вместо O(n^2 * n log n).
/// Используем HashSet для проверки уникальности за O(1) и сортируем один раз в конце.
/// Микрооптимизация: with_capacity для предаллокации.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());
    for &v in values {
        if seen.insert(v) {
            out.push(v);
        }
    }
    out.sort_unstable();
    out
}

/// Алгоритмическая оптимизация: O(n) вместо O(2^n).
/// Итеративная реализация чисел Фибоначчи.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}
