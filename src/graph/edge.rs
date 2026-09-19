use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Purchased,
    Interested,
    Similar,
    BelongsTo,
    Rated,
}

impl EdgeType {
    /// Multiplicador usado na pontuação de recomendações (relações mais fortes pesam mais).
    pub fn score_multiplier(self) -> f64 {
        match self {
            EdgeType::Purchased => 1.0,
            EdgeType::Similar => 0.9,
            EdgeType::Interested => 0.75,
            EdgeType::Rated => 0.85,
            EdgeType::BelongsTo => 0.5,
        }
    }
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeType::Purchased => write!(f, "Comprou"),
            EdgeType::Interested => write!(f, "Interesse"),
            EdgeType::Similar => write!(f, "Similar"),
            EdgeType::BelongsTo => write!(f, "Categoria"),
            EdgeType::Rated => write!(f, "Avaliou"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub to: crate::models::VertexId,
    pub edge_type: EdgeType,
    pub weight: f64,
}

impl Edge {
    pub fn new(to: crate::models::VertexId, edge_type: EdgeType, weight: f64) -> Self {
        Self {
            to,
            edge_type,
            weight,
        }
    }
}
