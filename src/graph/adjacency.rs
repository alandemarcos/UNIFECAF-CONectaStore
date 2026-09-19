use std::collections::HashMap;

use crate::models::VertexId;

use super::edge::Edge;
use super::vertex::{Vertex, VertexKind};

/// Grafo não direcionado modelado com lista de adjacência.
/// Cada vértice mantém lista de arestas para vizinhos; inserções duplicam
/// a aresta nas duas direções para facilitar BFS a partir de qualquer nó.
#[derive(Debug, Default)]
pub struct Graph {
    vertices: HashMap<VertexId, Vertex>,
    adjacency: HashMap<VertexId, Vec<Edge>>,
    next_vertex_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphError {
    VertexNotFound,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
            next_vertex_id: 1,
        }
    }

    fn alloc_vertex_id(&mut self) -> VertexId {
        let id = VertexId(self.next_vertex_id);
        self.next_vertex_id += 1;
        id
    }

    pub fn add_vertex(&mut self, kind: VertexKind) -> Result<VertexId, GraphError> {
        let id = self.alloc_vertex_id();
        let vertex = match kind {
            VertexKind::Customer(cid) => Vertex::customer(id, cid),
            VertexKind::Product(pid) => Vertex::product(id, pid),
            VertexKind::Category(catid) => Vertex::category(id, catid),
        };
        self.vertices.insert(id, vertex);
        self.adjacency.entry(id).or_default();
        Ok(id)
    }

    pub fn get_vertex(&self, id: VertexId) -> Option<&Vertex> {
        self.vertices.get(&id)
    }

    pub fn vertices(&self) -> impl Iterator<Item = &Vertex> {
        self.vertices.values()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(|edges| edges.len()).sum::<usize>() / 2
    }

    pub fn neighbors(&self, from: VertexId) -> Option<&[Edge]> {
        self.adjacency.get(&from).map(|v| v.as_slice())
    }

    pub fn add_undirected_edge(
        &mut self,
        from: VertexId,
        to: VertexId,
        edge: Edge,
    ) -> Result<(), GraphError> {
        if !self.vertices.contains_key(&from) || !self.vertices.contains_key(&to) {
            return Err(GraphError::VertexNotFound);
        }
        let reverse = Edge {
            to: from,
            edge_type: edge.edge_type,
            weight: edge.weight,
        };
        self.adjacency.get_mut(&from).expect("vertex exists").push(edge);
        self.adjacency.get_mut(&to).expect("vertex exists").push(reverse);
        Ok(())
    }

    pub fn add_directed_edge(
        &mut self,
        from: VertexId,
        to: VertexId,
        edge: Edge,
    ) -> Result<(), GraphError> {
        if !self.vertices.contains_key(&from) || !self.vertices.contains_key(&to) {
            return Err(GraphError::VertexNotFound);
        }
        self.adjacency.get_mut(&from).expect("vertex exists").push(edge);
        Ok(())
    }

    pub fn adjacency_lists(&self) -> &HashMap<VertexId, Vec<Edge>> {
        &self.adjacency
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::EdgeType;
    use crate::models::{CustomerId, ProductId};

    #[test]
    fn insert_vertices_and_edges() {
        let mut g = Graph::new();
        let c = g
            .add_vertex(VertexKind::Customer(CustomerId(1)))
            .expect("customer");
        let p = g.add_vertex(VertexKind::Product(ProductId(1))).expect("product");
        g.add_undirected_edge(
            c,
            p,
            Edge::new(p, EdgeType::Purchased, 1.0),
        )
        .expect("edge");
        assert_eq!(g.vertex_count(), 2);
        assert_eq!(g.edge_count(), 1);
    }
}
