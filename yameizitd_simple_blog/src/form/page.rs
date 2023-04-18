use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub records: Vec<T>,
    pub num: Option<i64>,
    pub size: Option<i64>,
    pub first: bool,
    pub last: bool,
    pub count: i64,
    pub total: i64,
    pub pages: i64,
}

impl<T> Page<T> {
    pub fn limit(&self) -> Option<i64> {
        return self.size;
    }

    pub fn offset(&self) -> Option<i64> {
        if let Some(num) = self.num {
            if let Some(size) = self.size {
                return Some((num - 1) * size);
            }
        }
        return None;
    }
}
