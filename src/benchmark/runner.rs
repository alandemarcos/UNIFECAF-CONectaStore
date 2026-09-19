use std::time::Instant;

use crate::models::CustomerId;
use crate::recommendation::RecommendationEngine;

use super::synthetic::build_synthetic_store;

/// Volumes padrão do trabalho (produtos no grafo sintético).
pub const BENCHMARK_VOLUMES: [usize; 3] = [100, 1_000, 10_000];

/// Profundidade máxima do BFS na medição (mesma operação da recomendação por cliente).
pub const BENCHMARK_MAX_DEPTH: usize = 6;

/// Limite de recomendações retornadas por execução medida.
pub const BENCHMARK_RECOMMENDATION_LIMIT: usize = 20;

/// Repetições por volume para estabilizar a medição.
pub const BENCHMARK_REPETITIONS: u32 = 10;

#[derive(Debug, Clone)]
pub struct BenchmarkRow {
    pub volume: usize,
    pub repetitions: u32,
    pub avg_micros: u128,
    pub min_micros: u128,
    pub max_micros: u128,
    pub total_micros: u128,
}

pub struct BenchmarkConfig {
    pub volumes: Vec<usize>,
    pub repetitions: u32,
    pub max_depth: usize,
    pub limit: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            volumes: BENCHMARK_VOLUMES.to_vec(),
            repetitions: BENCHMARK_REPETITIONS,
            max_depth: BENCHMARK_MAX_DEPTH,
            limit: BENCHMARK_RECOMMENDATION_LIMIT,
        }
    }
}

fn measure_recommendation_micros(
    volume: usize,
    repetitions: u32,
    max_depth: usize,
    limit: usize,
) -> BenchmarkRow {
    let store = build_synthetic_store(volume);
    let customer = CustomerId(1);
    let mut samples = Vec::with_capacity(repetitions as usize);

    for _ in 0..repetitions {
        let start = Instant::now();
        let _ = RecommendationEngine::for_customer(&store, customer, max_depth, limit);
        samples.push(start.elapsed().as_micros());
    }

    let total_micros: u128 = samples.iter().sum();
    let min_micros = *samples.iter().min().expect("repetitions > 0");
    let max_micros = *samples.iter().max().expect("repetitions > 0");
    let avg_micros = total_micros / repetitions as u128;

    BenchmarkRow {
        volume,
        repetitions,
        avg_micros,
        min_micros,
        max_micros,
        total_micros,
    }
}

pub fn run_benchmark(volumes: &[usize]) -> Vec<BenchmarkRow> {
    let config = BenchmarkConfig {
        volumes: volumes.to_vec(),
        ..Default::default()
    };
    run_benchmark_with_config(&config)
}

pub fn run_benchmark_with_config(config: &BenchmarkConfig) -> Vec<BenchmarkRow> {
    config
        .volumes
        .iter()
        .map(|&volume| {
            measure_recommendation_micros(
                volume,
                config.repetitions,
                config.max_depth,
                config.limit,
            )
        })
        .collect()
}

pub fn print_benchmark_table(rows: &[BenchmarkRow]) {
    let cfg = BenchmarkConfig::default();
    println!("\n# Benchmark CONectaStore\n");
    println!("Operação: recomendação por cliente (BFS + pontuação + ordenação)");
    println!(
        "Grafo sintético: cadeia de similaridade entre produtos + compra do cliente no primeiro produto"
    );
    println!("Profundidade máxima: {}", cfg.max_depth);
    println!("Limite de recomendações: {}", cfg.limit);
    println!("Repetições por volume: {}", cfg.repetitions);
    println!("Unidade: microssegundos (µs)\n");
    println!(
        "{:<14} {:>12} {:>12} {:>12}",
        "Produtos", "Média (µs)", "Mín (µs)", "Máx (µs)"
    );
    for row in rows {
        println!(
            "{:<14} {:>12} {:>12} {:>12}",
            row.volume, row.avg_micros, row.min_micros, row.max_micros
        );
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
            assert_eq!(row.repetitions, BENCHMARK_REPETITIONS);
            assert!(row.avg_micros < 60_000_000);
        }
    }
}
