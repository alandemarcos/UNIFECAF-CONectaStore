use std::collections::HashSet;

use crate::graph::bfs;
use crate::graph::VertexKind;
use crate::models::{CustomerId, ProductId};
use crate::repository::Store;

use super::scoring::{score_recommendation, ScoredRecommendation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationError {
    CustomerNotFound,
    ProductNotFound,
    NoRecommendations,
}

pub struct RecommendationEngine;

impl RecommendationEngine {
    pub fn for_customer(
        store: &Store,
        customer_id: CustomerId,
        max_depth: usize,
        limit: usize,
    ) -> Result<Vec<ScoredRecommendation>, RecommendationError> {
        let start = store
            .customer_vertex(customer_id)
            .ok_or(RecommendationError::CustomerNotFound)?;

        let exclude: HashSet<ProductId> = store
            .get_customer(customer_id)
            .map(|c| c.purchased_product_ids.iter().copied().collect())
            .unwrap_or_default();

        Self::collect_product_recommendations(store, start, max_depth, limit, exclude, None)
    }

    pub fn for_product(
        store: &Store,
        product_id: ProductId,
        max_depth: usize,
        limit: usize,
    ) -> Result<Vec<ScoredRecommendation>, RecommendationError> {
        let start = store
            .product_vertex(product_id)
            .ok_or(RecommendationError::ProductNotFound)?;

        let mut exclude = HashSet::new();
        exclude.insert(product_id);

        Self::collect_product_recommendations(
            store,
            start,
            max_depth,
            limit,
            exclude,
            Some(product_id),
        )
    }

    fn collect_product_recommendations(
        store: &Store,
        start: crate::models::VertexId,
        max_depth: usize,
        limit: usize,
        exclude_products: HashSet<ProductId>,
        exclude_self: Option<ProductId>,
    ) -> Result<Vec<ScoredRecommendation>, RecommendationError> {
        let steps = bfs(store.graph(), start, max_depth);
        let mut seen_products = HashSet::new();
        let mut scored = Vec::new();

        for step in steps {
            let Some(vertex) = store.graph().get_vertex(step.vertex) else {
                continue;
            };
            let VertexKind::Product(pid) = vertex.kind else {
                continue;
            };

            if exclude_self == Some(pid) {
                continue;
            }
            if exclude_products.contains(&pid) {
                continue;
            }
            if !seen_products.insert(pid) {
                continue;
            }

            let weight = step.via_edge.as_ref().map(|e| e.weight).unwrap_or(0.5);
            let edge_type = step
                .via_edge
                .as_ref()
                .map(|e| e.edge_type)
                .unwrap_or(crate::graph::EdgeType::BelongsTo);

            let score = score_recommendation(weight, edge_type, step.depth);
            scored.push(ScoredRecommendation {
                product_id: pid,
                score,
                depth: step.depth,
            });
        }

        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(limit);

        if scored.is_empty() {
            return Err(RecommendationError::NoRecommendations);
        }

        Ok(scored)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ProductId;
    use crate::repository::Store;

    fn setup_store_with_chain() -> (Store, CustomerId, ProductId, ProductId) {
        let mut store = Store::new();
        let cat = store.register_category("Informática").expect("cat");
        let notebook_a = store
            .register_product("Notebook A", cat.id, 3000.0, "A")
            .expect("na");
        let notebook_b = store
            .register_product("Notebook B", cat.id, 3200.0, "B")
            .expect("nb");
        let mouse = store
            .register_product("Mouse X", cat.id, 150.0, "X")
            .expect("mouse");
        let customer = store.register_customer("Cliente 1").expect("c1");
        store
            .link_purchase(customer.id, notebook_a.id, 1.0)
            .expect("purchase");
        store
            .link_similar_products(notebook_a.id, notebook_b.id, 0.9)
            .expect("sim");
        store
            .link_similar_products(notebook_b.id, mouse.id, 0.7)
            .expect("sim2");
        (store, customer.id, notebook_a.id, mouse.id)
    }

    #[test]
    fn recommends_related_products_for_customer() {
        let (store, customer, _, mouse) = setup_store_with_chain();
        let recs = RecommendationEngine::for_customer(&store, customer, 4, 10).expect("recs");
        let ids: Vec<_> = recs.iter().map(|r| r.product_id).collect();
        assert!(ids.contains(&mouse));
    }

    #[test]
    fn no_duplicate_recommendations() {
        let (store, customer, _, _) = setup_store_with_chain();
        let recs = RecommendationEngine::for_customer(&store, customer, 4, 10).expect("recs");
        let mut set = HashSet::new();
        for r in recs {
            assert!(set.insert(r.product_id));
        }
    }

    #[test]
    fn product_recommendation_excludes_self() {
        let (store, _, notebook_a, _) = setup_store_with_chain();
        let recs =
            RecommendationEngine::for_product(&store, notebook_a, 4, 10).expect("recs");
        assert!(!recs.iter().any(|r| r.product_id == notebook_a));
    }

    #[test]
    fn isolated_customer_no_recommendations() {
        let mut s = Store::new();
        let cat = s.register_category("Y").expect("c");
        let p = s
            .register_product("Solitário", cat.id, 1.0, "sem links")
            .expect("p");
        let c = s.register_customer("Isolado").expect("cust");
        let err = RecommendationEngine::for_customer(&s, c.id, 2, 5);
        assert!(matches!(err, Err(RecommendationError::NoRecommendations)));
        let err2 = RecommendationEngine::for_product(&s, p.id, 2, 5);
        assert!(matches!(err2, Err(RecommendationError::NoRecommendations)));
    }
}
