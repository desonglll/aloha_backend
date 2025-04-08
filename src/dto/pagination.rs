use crate::configuration::get_configuration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct Pagination {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub total: Option<i64>,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}

impl Pagination {
    pub fn new(page: Option<usize>, size: Option<usize>, total: Option<i64>) -> Self {
        let config = get_configuration().unwrap(); // 使用unwrap()时要确保不会出错

        let page = page.unwrap_or(1); // 默认值 1
        let size = size.unwrap_or(10); // 默认值 10
        let total = total.unwrap_or(0) as usize;

        // 计算上一页和下一页的逻辑
        let prev_page = if page > 1 {
            Some(format!(
                "{}:{}/{}?page={}&size={}",
                config.application.base_url,
                config.application.port,
                config.routes.user_groups,
                page - 1,
                size
            ))
        } else {
            None // 没有上一页时返回 None
        };

        let next_page = if page * size < total {
            Some(format!(
                "{}:{}/{}?page={}&size={}",
                config.application.base_url,
                config.application.port,
                config.routes.user_groups,
                page + 1,
                size
            ))
        } else {
            None // 没有下一页时返回 None
        };

        Self {
            page: Some(page),
            size: Some(size),
            total: Some(total as i64), // 保持 `total` 的 i64 类型
            prev_page,
            next_page,
        }
    }

    pub fn default_p() -> Self {
        Pagination {
            page: Some(1),
            size: Some(10),
            total: Some(0),
            prev_page: Option::from(String::new()),
            next_page: Option::from(String::new()),
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
    pub fn total(&self) -> i64 {
        self.total.unwrap_or(0)
    }
}
