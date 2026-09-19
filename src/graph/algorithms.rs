use std::collections::{HashSet, VecDeque};

use crate::models::VertexId;

use super::adjacency::Graph;
use super::edge::Edge;

#[derive(Debug, Clone, PartialEq)]
pub struct BfsStep {
    pub vertex: VertexId,
    pub depth: usize,
    pub via_edge: Option<Edge>,
}

/// BFS a partir de `start`, limitado por `max_depth`.
/// Usa VecDeque como fila e HashSet de visitados para evitar ciclos e revisitas.
/// Retorna vértices alcançados (exceto o start) com profundidade e aresta do primeiro acesso.
pub fn bfs(graph: &Graph, start: VertexId, max_depth: usize) -> Vec<BfsStep> {
    let mut result = Vec::new();
    if graph.get_vertex(start).is_none() {
        return result;
    }

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0usize, None));

    while let Some((current, depth, via)) = queue.pop_front() {
        if current != start {
            result.push(BfsStep {
                vertex: current,
                depth,
                via_edge: via,
            });
        }

        if depth >= max_depth {
            continue;
        }

        let Some(neighbors) = graph.neighbors(current) else {
            continue;
        };

        for edge in neighbors {
            if visited.insert(edge.to) {
                queue.push_back((edge.to, depth + 1, Some(edge.clone())));
            }
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq)]
pub struct DfsStep {
    pub vertex: VertexId,
    pub depth: usize,
}

/// DFS iterativo com limite de profundidade (demonstração complementar ao BFS).
pub fn dfs(graph: &Graph, start: VertexId, max_depth: usize) -> Vec<DfsStep> {
    let mut result = Vec::new();
    if graph.get_vertex(start).is_none() {
        return result;
    }

    let mut visited = HashSet::new();
    let mut stack = vec![(start, 0usize)];

    while let Some((current, depth)) = stack.pop() {
        if !visited.insert(current) {
            continue;
        }
        if current != start {
            result.push(DfsStep {
                vertex: current,
                depth,
            });
        }
        if depth >= max_depth {
            continue;
        }
        if let Some(neighbors) = graph.neighbors(current) {
            for edge in neighbors.iter().rev() {
                if !visited.contains(&edge.to) {
                    stack.push((edge.to, depth + 1));
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Edge, EdgeType, VertexKind};
    use crate::models::{CustomerId, ProductId};

    fn chain_graph() -> (Graph, VertexId, VertexId, VertexId) {
        let mut g = Graph::new();
        let c = g
            .add_vertex(VertexKind::Customer(CustomerId(1)))
            .expect("c");
        let p1 = g.add_vertex(VertexKind::Product(ProductId(1))).expect("p1");
        let p2 = g.add_vertex(VertexKind::Product(ProductId(2))).expect("p2");
        g.add_undirected_edge(c, p1, Edge::new(p1, EdgeType::Purchased, 1.0))
            .expect("e1");
        g.add_undirected_edge(p1, p2, Edge::new(p2, EdgeType::Similar, 0.8))
            .expect("e2");
        (g, c, p1, p2)
    }

    #[test]
    fn bfs_finds_products_at_depth() {
        let (g, customer, _, p2) = chain_graph();
        let steps = bfs(&g, customer, 3);
        assert!(steps.iter().any(|s| s.vertex == p2));
    }

    #[test]
    fn bfs_respects_max_depth() {
        let (g, customer, _, p2) = chain_graph();
        let steps = bfs(&g, customer, 1);
        assert!(!steps.iter().any(|s| s.vertex == p2));
    }

    #[test]
    fn bfs_handles_cycle_without_infinite_loop() {
        let mut g = Graph::new();
        let p1 = g.add_vertex(VertexKind::Product(ProductId(1))).expect("p1");
        let p2 = g.add_vertex(VertexKind::Product(ProductId(2))).expect("p2");
        let p3 = g.add_vertex(VertexKind::Product(ProductId(3))).expect("p3");
        g.add_undirected_edge(p1, p2, Edge::new(p2, EdgeType::Similar, 1.0))
            .expect("e1");
        g.add_undirected_edge(p2, p3, Edge::new(p3, EdgeType::Similar, 1.0))
            .expect("e2");
        g.add_undirected_edge(p3, p1, Edge::new(p1, EdgeType::Similar, 1.0))
            .expect("e3");
        let steps = bfs(&g, p1, 10);
        assert!(steps.len() <= 2);
    }

    #[test]
    fn dfs_handles_cycle_without_infinite_loop() {
        let mut g = Graph::new();
        let p1 = g.add_vertex(VertexKind::Product(ProductId(1))).expect("p1");
        let p2 = g.add_vertex(VertexKind::Product(ProductId(2))).expect("p2");
        g.add_undirected_edge(p1, p2, Edge::new(p2, EdgeType::Similar, 1.0))
            .expect("e");
        g.add_undirected_edge(p2, p1, Edge::new(p1, EdgeType::Similar, 1.0))
            .expect("e2");
        let steps = dfs(&g, p1, 10);
        assert!(steps.len() <= 1);
    }
}
