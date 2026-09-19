use crate::repository::Store;

/// Gera catálogo sintético com cadeias de similaridade para medir recomendação em escala.
pub fn build_synthetic_store(product_count: usize) -> Store {
    let mut store = Store::new();
    let cat = store
        .register_category("Eletrônicos")
        .expect("category");

    let mut prev = store
        .register_product("Produto-0", cat.id, 100.0, "seed")
        .expect("p0");

    for i in 1..product_count {
        let p = store
            .register_product(
                format!("Produto-{i}"),
                cat.id,
                100.0 + i as f64,
                "synthetic",
            )
            .expect("product");
        store
            .link_similar_products(prev.id, p.id, 0.85)
            .expect("similar");
        prev = p;
    }

    let customer = store
        .register_customer("Cliente Benchmark")
        .expect("customer");
    if let Some(first) = store.list_products().first() {
        store
            .link_purchase(customer.id, first.id, 1.0)
            .expect("purchase");
    }

    store
}
