pub mod category;
pub mod post;

pub use category::*;
pub use post::*;

pub struct DynamicQuery {
    pub query_sql: String,
    pub conditional: bool,
    pub condition_index: u64,
    pub limit: i64,
    pub offset: i64,
}

impl DynamicQuery {
    fn builder(base_sql: &str) -> Self {
        DynamicQuery {
            query_sql: base_sql.to_string(),
            conditional: false,
            condition_index: 1,
            limit: 10,
            offset: 0,
        }
    }

    fn and<T>(mut self, column_name: &str, expression: &str, condition: Option<T>) -> Self {
        if let Some(condition) = condition {
            if self.conditional {
                self.query_sql.push_str(&format!(
                    " and {} {} ${}",
                    column_name, expression, self.condition_index
                ));
                self.condition_index += 1;
            } else {
                self.query_sql.push_str(&format!(
                    " where {} {} ${}",
                    column_name, expression, self.condition_index
                ));
                self.conditional = true;
                self.condition_index += 1;
            }
        }
        self
    }

    fn or<T>(mut self, column_name: &str, expression: &str, condition: Option<T>) -> Self {
        if let Some(condition) = condition {
            if self.conditional {
                self.query_sql.push_str(&format!(
                    " or {} {} ${}",
                    column_name, expression, self.condition_index
                ));
                self.condition_index += 1;
            } else {
                self.query_sql.push_str(&format!(
                    " where {} {} ${}",
                    column_name, expression, self.condition_index
                ));
                self.conditional = true;
                self.condition_index += 1;
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
        if let Some(page_num) = page_num {
            if let Some(page_size) = page_size {
                self.query_sql.push_str(&format!(
                    " limit ${} offset ${}",
                    self.condition_index,
                    self.condition_index + 1
                ));
                self.condition_index += 2;
                self.limit = page_size;
                self.offset = (page_num - 1) * page_size;
            }
        } else {
            self.query_sql.push_str(&format!(
                " limit ${} offset ${}",
                self.condition_index,
                self.condition_index + 1
            ));
            self.condition_index += 2;
            self.limit = default_size;
            self.offset = (defalut_num - 1) * default_size;
        }
        self
    }

    fn build_sql(&self) -> &String {
        &self.query_sql
    }
}
