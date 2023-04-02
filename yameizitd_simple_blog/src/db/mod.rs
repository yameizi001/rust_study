pub mod category;
pub mod post;

pub use category::*;
pub use post::*;

use sqlx::{
    database::HasArguments,
    query::{Query, QueryAs},
    Database, Encode, FromRow, Type,
};

pub struct DynamicQuery<'args, DB: Database> {
    pub inner_query_builder: sqlx::QueryBuilder<'args, DB>,
    pub conditional: bool,
    pub condition_index: u64,
}

impl<'args, DB> DynamicQuery<'args, DB>
where
    DB: Database,
    i64: 'args + Encode<'args, DB> + Send + Type<DB>,
{
    pub fn builder(base_sql: impl Into<String>) -> Self {
        DynamicQuery {
            inner_query_builder: sqlx::QueryBuilder::new(base_sql),
            conditional: false,
            condition_index: 1,
        }
    }

    fn and<T>(mut self, column_name: &str, expression: &str, condition: Option<T>) -> Self
    where
        T: 'args + Encode<'args, DB> + Send + Type<DB>,
    {
        if let Some(condition) = condition {
            if self.conditional {
                self.inner_query_builder
                    .push(&format!(" and {} {} ", column_name, expression));
                self.inner_query_builder.push_bind(condition);
            } else {
                self.inner_query_builder
                    .push(&format!(" where {} {} ", column_name, expression));
                self.conditional = true;
                self.inner_query_builder.push_bind(condition);
            }
        }
        self
    }

    fn or<T>(mut self, column_name: &str, expression: &str, condition: Option<T>) -> Self
    where
        T: 'args + Encode<'args, DB> + Send + Type<DB>,
    {
        if let Some(condition) = condition {
            if self.conditional {
                self.inner_query_builder
                    .push(&format!(" or {} {} ", column_name, expression));
                self.inner_query_builder.push_bind(condition);
            } else {
                self.inner_query_builder
                    .push(&format!(" where {} {} ", column_name, expression));
                self.conditional = true;
                self.inner_query_builder.push_bind(condition);
            }
        }
        self
    }

    fn page(self, page_num: Option<i64>, page_size: Option<i64>) -> Self {
        self.page_with_default(page_num, page_size, 1, 10)
    }

    fn page_with_default(
        mut self,
        page_num: Option<i64>,
        page_size: Option<i64>,
        defalut_num: i64,
        default_size: i64,
    ) -> Self {
        let page_num = page_num.or(Some(defalut_num));
        let page_size = page_size.or(Some(default_size));
        if let Some(page_num) = page_num {
            if let Some(page_size) = page_size {
                self.inner_query_builder.push(" limit ");
                self.inner_query_builder.push_bind(page_size);
                self.inner_query_builder.push(" offset ");
                self.inner_query_builder
                    .push_bind((page_num - 1) * page_size);
            }
        }
        self
    }

    pub fn build(&mut self) -> Query<'_, DB, <DB as HasArguments<'args>>::Arguments> {
        self.inner_query_builder.build()
    }

    pub fn build_as<'q, T: FromRow<'q, DB::Row>>(
        &'q mut self,
    ) -> QueryAs<'q, DB, T, <DB as HasArguments<'args>>::Arguments> {
        r#"sqlx-page-helper"#;
        self.inner_query_builder.build_query_as()
    }
}
