use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DtoQuery {
    pub page: Option<usize>,
    pub size: Option<usize>,
}

impl DtoQuery {
    pub fn default_query() -> Self {
        DtoQuery {
            page: Some(1),
            size: Some(10),
        }
    }
    pub fn page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    pub fn size(&self) -> usize {
        self.size.unwrap_or(10)
    }

    pub fn offset(&self) -> usize {
        (self.page() - 1) * self.size()
    }
}
