pub mod category;
pub mod post;

pub use category::*;
pub use post::*;

pub fn limit(page_size: Option<i64>) -> Option<i64> {
    page_size
}

pub fn offset(page_num: Option<i64>, page_size: Option<i64>) -> Option<i64> {
    if let Some(page_num) = page_num {
        if let Some(page_size) = page_size {
            return Some((page_num - 1) * page_size);
        }
    }
    None
}
