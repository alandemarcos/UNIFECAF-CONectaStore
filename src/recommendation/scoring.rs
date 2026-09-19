use crate::graph::EdgeType;
use crate::models::ProductId;

#[derive(Debug, Clone, PartialEq)]
pub struct ScoredRecommendation {
    pub product_id: ProductId,
    pub score: f64,
    pub depth: usize,
}

/// Pontuação: combina peso da aresta, tipo de relação e proximidade (profundidade no BFS).
///
/// score = (weight * type_multiplier) / depth   (depth mínimo 1)
///
/// Produtos mais próximos e conectados por relações fortes aparecem primeiro.
pub fn score_recommendation(weight: f64, edge_type: EdgeType, depth: usize) -> f64 {
    let depth = depth.max(1) as f64;
    (weight * edge_type.score_multiplier()) / depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closer_and_stronger_scores_higher() {
        let near = score_recommendation(1.0, EdgeType::Similar, 1);
        let far = score_recommendation(1.0, EdgeType::Similar, 3);
        assert!(near > far);
    }
}
