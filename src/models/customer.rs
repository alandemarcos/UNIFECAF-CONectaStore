use super::{CustomerId, ProductId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    /// Produtos já adquiridos ou explicitamente relacionados ao cliente.
    pub purchased_product_ids: Vec<ProductId>,
}

impl Customer {
    pub fn new(id: CustomerId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            purchased_product_ids: Vec::new(),
        }
    }

    pub fn with_purchases(mut self, product_ids: Vec<ProductId>) -> Self {
        self.purchased_product_ids = product_ids;
        self
    }
}
