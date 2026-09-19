//! Executa benchmark sem menu interativo: `cargo run --bin benchmark`

use conectastore::benchmark::{print_benchmark_table, run_benchmark};

fn main() {
    let rows = run_benchmark(&[100, 1_000, 10_000]);
    print_benchmark_table(&rows);
}
