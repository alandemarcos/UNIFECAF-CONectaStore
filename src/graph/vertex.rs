use crate::models::{CategoryId, CustomerId, ProductId, VertexId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexKind {
    Customer(CustomerId),
    Product(ProductId),
    Category(CategoryId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vertex {
    pub id: VertexId,
    pub kind: VertexKind,
}

impl Vertex {
    pub fn customer(id: VertexId, customer_id: CustomerId) -> Self {
        Self {
            id,
            kind: VertexKind::Customer(customer_id),
        }
    }

    pub fn product(id: VertexId, product_id: ProductId) -> Self {
        Self {
            id,
            kind: VertexKind::Product(product_id),
        }
    }

    pub fn category(id: VertexId, category_id: CategoryId) -> Self {
        Self {
            id,
            kind: VertexKind::Category(category_id),
        }
    }

    pub fn is_product(&self) -> bool {
        matches!(self.kind, VertexKind::Product(_))
    }

    pub fn product_id(&self) -> Option<ProductId> {
        match self.kind {
            VertexKind::Product(pid) => Some(pid),
            _ => None,
        }
    }
}
