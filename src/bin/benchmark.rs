//! Executa benchmark sem menu interativo: `cargo run --release --bin benchmark`

use conectastore::benchmark::{print_benchmark_table, run_benchmark, BENCHMARK_VOLUMES};

fn main() {
    let rows = run_benchmark(&BENCHMARK_VOLUMES);
    print_benchmark_table(&rows);
}
