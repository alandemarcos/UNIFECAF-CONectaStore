use std::collections::HashMap;

use crate::graph::{Edge, EdgeType, Graph, VertexKind};
use crate::models::{Category, CategoryId, Customer, CustomerId, Product, ProductId, VertexId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    CategoryNotFound,
    ProductNotFound,
    CustomerNotFound,
    CustomerAlreadyExists,
    InvalidInput,
    GraphError(crate::graph::GraphError),
}

impl From<crate::graph::GraphError> for StoreError {
    fn from(value: crate::graph::GraphError) -> Self {
        StoreError::GraphError(value)
    }
}

/// Repositório: HashMaps para entidades + grafo de relações.
#[derive(Debug)]
pub struct Store {
    products: HashMap<ProductId, Product>,
    customers: HashMap<CustomerId, Customer>,
    categories: HashMap<CategoryId, Category>,
    graph: Graph,
    product_vertices: HashMap<ProductId, VertexId>,
    customer_vertices: HashMap<CustomerId, VertexId>,
    category_vertices: HashMap<CategoryId, VertexId>,
    next_product_id: u64,
    next_customer_id: u64,
    next_category_id: u64,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            products: HashMap::new(),
            customers: HashMap::new(),
            categories: HashMap::new(),
            graph: Graph::new(),
            product_vertices: HashMap::new(),
            customer_vertices: HashMap::new(),
            category_vertices: HashMap::new(),
            next_product_id: 1,
            next_customer_id: 1,
            next_category_id: 1,
        }
    }
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    // --- Categories ---

    pub fn register_category(&mut self, name: impl Into<String>) -> Result<Category, StoreError> {
        let id = CategoryId(self.next_category_id);
        self.next_category_id += 1;
        let category = Category::new(id, name);
        self.categories.insert(id, category.clone());
        let vertex = self.graph.add_vertex(VertexKind::Category(id))?;
        self.category_vertices.insert(id, vertex);
        Ok(category)
    }

    pub fn get_category(&self, id: CategoryId) -> Option<&Category> {
        self.categories.get(&id)
    }

    pub fn list_categories(&self) -> Vec<&Category> {
        let mut v: Vec<_> = self.categories.values().collect();
        v.sort_by_key(|c| c.id.0);
        v
    }

    // --- Products ---

    pub fn register_product(
        &mut self,
        name: impl Into<String>,
        category_id: CategoryId,
        price: f64,
        description: impl Into<String>,
    ) -> Result<Product, StoreError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(StoreError::InvalidInput);
        }
        if !price.is_finite() || price < 0.0 {
            return Err(StoreError::InvalidInput);
        }
        if !self.categories.contains_key(&category_id) {
            return Err(StoreError::CategoryNotFound);
        }
        let id = ProductId(self.next_product_id);
        self.next_product_id += 1;
        let product = Product::new(id, name, category_id, price, description.into());
        self.products.insert(id, product.clone());
        let vertex = self.graph.add_vertex(VertexKind::Product(id))?;
        self.product_vertices.insert(id, vertex);

        if let Some(&cat_vertex) = self.category_vertices.get(&category_id) {
            self.graph.add_undirected_edge(
                vertex,
                cat_vertex,
                Edge::new(cat_vertex, EdgeType::BelongsTo, 0.6),
            )?;
        }

        Ok(product)
    }

    pub fn get_product(&self, id: ProductId) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn list_products(&self) -> Vec<&Product> {
        let mut v: Vec<_> = self.products.values().collect();
        v.sort_by_key(|p| p.id.0);
        v
    }

    // --- Customers ---

    pub fn register_customer(&mut self, name: impl Into<String>) -> Result<Customer, StoreError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(StoreError::InvalidInput);
        }
        let id = CustomerId(self.next_customer_id);
        self.next_customer_id += 1;
        self.insert_customer(id, name)
    }

    /// Cadastra cliente com ID explícito (CLI). Rejeita ID duplicado.
    pub fn register_customer_with_id(
        &mut self,
        id: CustomerId,
        name: impl Into<String>,
    ) -> Result<Customer, StoreError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(StoreError::InvalidInput);
        }
        if self.customers.contains_key(&id) {
            return Err(StoreError::CustomerAlreadyExists);
        }
        if id.0 >= self.next_customer_id {
            self.next_customer_id = id.0 + 1;
        }
        self.insert_customer(id, name)
    }

    fn insert_customer(&mut self, id: CustomerId, name: String) -> Result<Customer, StoreError> {
        let customer = Customer::new(id, name);
        self.customers.insert(id, customer.clone());
        let vertex = self.graph.add_vertex(VertexKind::Customer(id))?;
        self.customer_vertices.insert(id, vertex);
        Ok(customer)
    }

    pub fn get_customer(&self, id: CustomerId) -> Option<&Customer> {
        self.customers.get(&id)
    }

    pub fn list_customers(&self) -> Vec<&Customer> {
        let mut v: Vec<_> = self.customers.values().collect();
        v.sort_by_key(|c| c.id.0);
        v
    }

    pub fn product_vertex(&self, product_id: ProductId) -> Option<VertexId> {
        self.product_vertices.get(&product_id).copied()
    }

    pub fn customer_vertex(&self, customer_id: CustomerId) -> Option<VertexId> {
        self.customer_vertices.get(&customer_id).copied()
    }

    // --- Connections ---

    pub fn link_purchase(
        &mut self,
        customer_id: CustomerId,
        product_id: ProductId,
        weight: f64,
    ) -> Result<(), StoreError> {
        let c_v = self
            .customer_vertices
            .get(&customer_id)
            .copied()
            .ok_or(StoreError::CustomerNotFound)?;
        let p_v = self
            .product_vertices
            .get(&product_id)
            .copied()
            .ok_or(StoreError::ProductNotFound)?;
        self.graph
            .add_undirected_edge(c_v, p_v, Edge::new(p_v, EdgeType::Purchased, weight))?;
        if let Some(customer) = self.customers.get_mut(&customer_id) {
            if !customer.purchased_product_ids.contains(&product_id) {
                customer.purchased_product_ids.push(product_id);
            }
        }
        Ok(())
    }

    pub fn link_interest(
        &mut self,
        customer_id: CustomerId,
        product_id: ProductId,
        weight: f64,
    ) -> Result<(), StoreError> {
        let c_v = self
            .customer_vertices
            .get(&customer_id)
            .copied()
            .ok_or(StoreError::CustomerNotFound)?;
        let p_v = self
            .product_vertices
            .get(&product_id)
            .copied()
            .ok_or(StoreError::ProductNotFound)?;
        self.graph
            .add_undirected_edge(c_v, p_v, Edge::new(p_v, EdgeType::Interested, weight))?;
        Ok(())
    }

    pub fn link_similar_products(
        &mut self,
        product_a: ProductId,
        product_b: ProductId,
        weight: f64,
    ) -> Result<(), StoreError> {
        let a_v = self
            .product_vertices
            .get(&product_a)
            .copied()
            .ok_or(StoreError::ProductNotFound)?;
        let b_v = self
            .product_vertices
            .get(&product_b)
            .copied()
            .ok_or(StoreError::ProductNotFound)?;
        self.graph
            .add_undirected_edge(a_v, b_v, Edge::new(b_v, EdgeType::Similar, weight))?;
        Ok(())
    }

    pub fn link_rating(
        &mut self,
        customer_id: CustomerId,
        product_id: ProductId,
        weight: f64,
    ) -> Result<(), StoreError> {
        let c_v = self
            .customer_vertices
            .get(&customer_id)
            .copied()
            .ok_or(StoreError::CustomerNotFound)?;
        let p_v = self
            .product_vertices
            .get(&product_id)
            .copied()
            .ok_or(StoreError::ProductNotFound)?;
        self.graph
            .add_undirected_edge(c_v, p_v, Edge::new(p_v, EdgeType::Rated, weight))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_get_product() {
        let mut store = Store::new();
        let cat = store.register_category("Informática").expect("cat");
        let p = store
            .register_product("Notebook A", cat.id, 3500.0, "16GB RAM")
            .expect("product");
        assert_eq!(store.get_product(p.id).unwrap().name, "Notebook A");
    }

    #[test]
    fn register_customer() {
        let mut store = Store::new();
        let c = store.register_customer("Maria").expect("customer");
        assert_eq!(store.get_customer(c.id).unwrap().name, "Maria");
    }

    #[test]
    fn register_customer_rejects_empty_name() {
        let mut store = Store::new();
        assert!(matches!(
            store.register_customer("   "),
            Err(StoreError::InvalidInput)
        ));
    }

    #[test]
    fn register_customer_rejects_duplicate_id() {
        let mut store = Store::new();
        store
            .register_customer_with_id(CustomerId(5), "A")
            .expect("first");
        assert!(matches!(
            store.register_customer_with_id(CustomerId(5), "B"),
            Err(StoreError::CustomerAlreadyExists)
        ));
    }

    #[test]
    fn product_not_found() {
        let store = Store::new();
        assert!(store.get_product(ProductId(999)).is_none());
    }
}
