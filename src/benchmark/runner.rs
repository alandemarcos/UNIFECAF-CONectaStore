use std::time::Instant;

use crate::models::CustomerId;
use crate::recommendation::RecommendationEngine;

use super::synthetic::build_synthetic_store;

#[derive(Debug, Clone)]
pub struct BenchmarkRow {
    pub volume: usize,
    pub elapsed_ms: u128,
}

pub fn run_benchmark(volumes: &[usize]) -> Vec<BenchmarkRow> {
    volumes
        .iter()
        .map(|&volume| {
            let store = build_synthetic_store(volume);
            let customer = CustomerId(1);
            let start = Instant::now();
            let _ = RecommendationEngine::for_customer(&store, customer, 6, 20);
            let elapsed = start.elapsed().as_millis();
            BenchmarkRow {
                volume,
                elapsed_ms: elapsed,
            }
        })
        .collect()
}

pub fn print_benchmark_table(rows: &[BenchmarkRow]) {
    println!("\n=== Benchmark de recomendação (BFS) ===");
    println!("{:<12} {}", "Volume", "Tempo (ms)");
    for row in rows {
        println!("{:<12} {}", row.volume, row.elapsed_ms);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_runs_three_volumes() {
        let rows = run_benchmark(&[10, 50, 100]);
        assert_eq!(rows.len(), 3);
        for row in rows {
            assert!(row.elapsed_ms < 60_000);
        }
    }
}
