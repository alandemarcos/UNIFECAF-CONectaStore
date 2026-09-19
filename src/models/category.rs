use super::CategoryId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
}

impl Category {
    pub fn new(id: CategoryId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
