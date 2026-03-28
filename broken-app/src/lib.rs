pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Исправлен off-by-one: было `0..=values.len()` (UB), стало безопасный итератор.
/// Удалён unsafe — он не нужен.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов.
/// Исправлена утечка памяти: убран raw-указатель, используется безопасный итератор.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|b| **b != 0).count()
}

/// Нормализация строки: убираем все виды пробельных символов и приводим к нижнему регистру.
/// Исправлено: `replace(' ', "")` удаляло только ASCII пробелы, не обрабатывало табы и т.п.
/// Теперь используем `split_whitespace` как в reference-app.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Корректное усреднение только положительных чисел.
/// Исправлена логическая ошибка: раньше суммировались все элементы и делились на общую длину.
pub fn average_positive(values: &[i64]) -> f64 {
    let positives: Vec<i64> = values.iter().copied().filter(|v| *v > 0).collect();
    if positives.is_empty() {
        return 0.0;
    }
    let sum: i64 = positives.iter().sum();
    sum as f64 / positives.len() as f64
}
