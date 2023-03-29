pub mod category;
pub mod post;

use std::marker::PhantomData;

pub use category::*;
pub use post::*;
use sqlx::{Database, Encode, Type};

#[derive(Debug)]
pub struct DynamicQuery {
    pub query_sql: String,
    pub conditional: bool,
    pub condition_index: u64,
}

impl DynamicQuery {
    pub fn builder(query_sql: &str) -> Self {
        DynamicQuery {
            query_sql: query_sql.to_string(),
            conditional: false,
            condition_index: 1,
        }
    }

    pub fn condition<T>(mut self, column_name: &str, condition: Option<T>) -> Self {
        if let Some(condition) = condition {
            if self.conditional {
                self.query_sql
                    .push_str(&format!(" and {} = ${}", column_name, self.condition_index));
                self.condition_index += 1;
            } else {
                self.query_sql.push_str(&format!(
                    " where {} = ${}",
                    column_name, self.condition_index
                ));
                self.conditional = true;
                self.condition_index += 1;
            }
        }
        self
    }

    pub fn page(mut self, num: Option<u64>, size: Option<u64>) -> Self {
        if let Some(num) = num {
            if let Some(size) = size {
                self.query_sql
                    .push_str(&format!(" limit {} offset {}", size, (num - 1) * size));
            }
        } else {
            self.query_sql
                .push_str(&format!(" limit {} offset {}", 10, 0));
        }
        self
    }

    pub fn build(self) -> String {
        self.query_sql
    }
}
