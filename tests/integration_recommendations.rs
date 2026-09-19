use conectastore::models::{CustomerId, ProductId};
use conectastore::recommendation::RecommendationEngine;
use conectastore::repository::Store;
use std::collections::HashSet;

fn build_test_store() -> (Store, CustomerId, ProductId, ProductId) {
    let mut store = Store::new();
    let cat = store.register_category("Informática").expect("cat");
    let p1 = store
        .register_product("Notebook A", cat.id, 3000.0, "A")
        .expect("p1");
    let p2 = store
        .register_product("Notebook B", cat.id, 3200.0, "B")
        .expect("p2");
    let p3 = store
        .register_product("Mouse X", cat.id, 150.0, "X")
        .expect("p3");
    let customer = store.register_customer("Cliente 1").expect("c");
    store
        .link_purchase(customer.id, p1.id, 1.0)
        .expect("purchase");
    store.link_similar_products(p1.id, p2.id, 0.9).expect("sim");
    store
        .link_similar_products(p2.id, p3.id, 0.8)
        .expect("sim2");
    (store, customer.id, p1.id, p3.id)
}

#[test]
fn integration_create_entities_connections_and_recommend() {
    let (store, customer, _, mouse) = build_test_store();
    let recs = RecommendationEngine::for_customer(&store, customer, 4, 10).expect("recs");
    assert!(recs.iter().any(|r| r.product_id == mouse));
}

#[test]
fn integration_no_duplicate_recommendations() {
    let (store, customer, _, _) = build_test_store();
    let recs = RecommendationEngine::for_customer(&store, customer, 4, 10).expect("recs");
    let mut seen = HashSet::new();
    for r in recs {
        assert!(seen.insert(r.product_id));
    }
}

#[test]
fn integration_product_recommendation_excludes_source() {
    let (store, _, notebook_a, _) = build_test_store();
    let recs = RecommendationEngine::for_product(&store, notebook_a, 4, 10).expect("recs");
    assert!(!recs.iter().any(|r| r.product_id == notebook_a));
}
