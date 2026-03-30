# Broken-app: исправление, верификация и оптимизация

## Описание задачи

Проект `broken-app` содержал 6 намеренно заложенных дефектов и 2 неоптимальных алгоритма.
Задача — найти и исправить все дефекты, подтвердить корректность инструментами верификации,
оптимизировать критичные участки и задокументировать результаты.

`reference-app` — эталонная реализация для сверки поведения.

Исходный архив: https://code.s3.yandex.net/middle-rust-blockchain/broken-app.zip

## Как собрать проект

```bash
cd broken-app
cargo build
```

## Как запустить тесты

```bash
cargo test
```

## Как прогнать Miri (проверка UB)

```bash
cargo +nightly miri test
```

## Как прогнать Valgrind (проверка утечек)

```bash
cargo test --tests --no-run
valgrind --leak-check=full target/debug/deps/integration-*
```

## Как запустить sanitizer'ы

```bash
# AddressSanitizer
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test --target x86_64-unknown-linux-gnu

# ThreadSanitizer
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test --target x86_64-unknown-linux-gnu
```

## Как запустить бенчмарки

```bash
cargo bench --bench baseline
cargo bench --bench criterion
```

## Как сгенерировать flamegraph

```bash
cargo flamegraph --bench baseline -o artifacts/flamegraph.svg
```

## Где лежат артефакты

```
broken-app/artifacts/
  baseline_before.txt       — бенчмарки до оптимизации
  baseline_after.txt        — бенчмарки после оптимизации
  criterion_before.txt      — criterion бенчмарки до оптимизации
  criterion_after.txt       — criterion бенчмарки после оптимизации
  criterion_reports/        — HTML-отчёты и SVG-графики criterion (gnuplot)
  flamegraph_before.svg     — flamegraph до оптимизации
  flamegraph_after.svg      — flamegraph после оптимизации
  cargo_test.txt            — лог прохождения тестов
  miri_test.txt             — лог Miri
  valgrind_before.txt       — лог Valgrind до исправлений (с утечкой)
  valgrind_after.txt        — лог Valgrind после исправлений (без утечек)
  asan_before.txt           — лог ASan до исправлений (UB в sum_even)
  asan_after.txt            — лог ASan после исправлений (чисто)
  tsan_before.txt           — лог TSan до исправлений (data race в concurrency)
  tsan_after.txt            — лог TSan после исправлений (чисто)
```

## Найденные и исправленные баги

### 1. Off-by-one в `sum_even` (UB)

- **Проблема**: цикл `0..=values.len()` обращался к `get_unchecked(values.len())` — доступ за границу среза.
- **Воспроизведение**: `cargo test` — abort с сообщением "unsafe precondition violated".
- **Инструмент**: cargo test, Miri.
- **Исправление**: заменён unsafe цикл на безопасный `values.iter().copied().filter(|v| v % 2 == 0).sum()`.
- **Тесты**: `sums_even_numbers`, `sums_even_empty`, `sums_even_all_odd`.

### 2. Утечка памяти в `leak_buffer`

- **Проблема**: `Box::into_raw` без последующего `Box::from_raw` — память не освобождалась.
- **Воспроизведение**: Valgrind показывал "definitely lost" bytes.
- **Инструмент**: Valgrind.
- **Исправление**: заменён на безопасный `input.iter().filter(|b| **b != 0).count()` без аллокаций.
- **Тесты**: `counts_non_zero_bytes`, `counts_non_zero_empty`, `counts_non_zero_all_zeros`.

### 3. Логическая ошибка в `average_positive`

- **Проблема**: суммировались все элементы и делились на общую длину вместо фильтрации положительных.
- **Воспроизведение**: `cargo test` — тест `averages_only_positive` падал с неверным результатом.
- **Инструмент**: cargo test.
- **Исправление**: фильтрация `v > 0`, деление на количество положительных.
- **Тесты**: `averages_only_positive`, `averages_empty`, `averages_all_negative`.

### 4. Неполная нормализация в `normalize`

- **Проблема**: `replace(' ', "")` удаляла только ASCII пробелы, не обрабатывала табуляции и другие whitespace.
- **Воспроизведение**: строки с `\t` нормализовались некорректно.
- **Инструмент**: сверка с reference-app.
- **Исправление**: замена на `split_whitespace().collect::<String>().to_lowercase()`.
- **Тесты**: `normalize_simple`, `normalize_tabs`, `normalize_multiple_spaces`.

### 5. Use-after-free в `use_after_free`

- **Проблема**: чтение `*raw` после `drop(Box::from_raw(raw))` — UB.
- **Воспроизведение**: Miri/ASan.
- **Инструмент**: Miri.
- **Исправление**: функция удалена полностью (не несёт полезной нагрузки).

### 6. Data race в `concurrency::race_increment`

- **Проблема**: `static mut COUNTER` без синхронизации — гонка данных при многопоточном доступе.
- **Воспроизведение**: TSan, результат `race_increment(1000, 4) != 4000`.
- **Инструмент**: TSan, cargo test.
- **Исправление**: замена `static mut` на `AtomicU64` с `Ordering::SeqCst`.
- **Тесты**: `race_increment_tests`.

## Оптимизации

### 1. Алгоритмическая: `slow_fib` — O(2^n) -> O(n)

- **Узкое место**: экспоненциальная рекурсия без мемоизации.
- **Изменение**: итеративная реализация с двумя переменными.
- **Эффект**: ~57,000x ускорение на fib(32) (5.6 мс -> 98 нс).

### 2. Алгоритмическая + микро: `slow_dedup` — O(n^2 * n log n) -> O(n log n)

- **Узкое место**: линейный поиск дубликатов O(n) + `sort_unstable()` на каждой вставке.
- **Изменение**: `HashSet` для O(1) проверки уникальности, одна сортировка в конце, `with_capacity` для предаллокации.
- **Эффект**: ~65x ускорение на 5000 дубликатов (9.3 мс -> 142 мкс).

## Сравнение до/после

| Функция | До (baseline) | После (baseline) | Ускорение |
|---|---|---|---|
| `slow_fib(32)` | 5.6 мс | 98 нс | **~57,000x** |
| `slow_dedup(5000 dup)` | 9.3 мс | 142 мкс | **~65x** |
| `sum_even(50000)` | 14.1 мкс | 14.3 мкс | без изменений |

## Верификация после оптимизации

- `cargo test` — 18/18 тестов проходят
- `cargo +nightly miri test` — 18/18, 0 UB
- `valgrind --leak-check=full` — 0 definitely lost, 0 indirectly lost
- Все бенчмарки воспроизводимы

## Ограничения и замечания

- Valgrind показывает "possibly lost: 48 bytes" — это внутренний thread-local стандартной библиотеки Rust, не связан с кодом проекта.
- `sum_even` не оптимизировался, так как уже работает за O(n) и является оптимальным.
- Имена функций `slow_fib` и `slow_dedup` сохранены для совместимости с тестами и бенчмарками.
