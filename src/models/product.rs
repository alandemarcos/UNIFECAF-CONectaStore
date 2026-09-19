use super::{CategoryId, ProductId};

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub category_id: CategoryId,
    pub price: f64,
    pub description: String,
}

impl Product {
    pub fn new(
        id: ProductId,
        name: impl Into<String>,
        category_id: CategoryId,
        price: f64,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            category_id,
            price,
            description: description.into(),
        }
    }
}
