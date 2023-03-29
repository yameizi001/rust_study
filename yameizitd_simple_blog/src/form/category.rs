use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateForm {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateForm {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryForm {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub id: Option<i64>,
    pub name: Option<String>,
}

impl QueryForm {
    pub fn conditions(&self) -> String {
        if None == self.page_num && None == self.page_size {
            format!("")
        } else {
            let id_condition = if let Some(id) = self.id {
                format!("id = $")
            } else {
                1
            };

            "".to_string()
        }
    }

    pub fn page(&self) -> String {
        let page_num = if let Some(page_num) = self.page_num {
            page_num
        } else {
            1
        };
        let page_size = if let Some(page_size) = self.page_size {
            page_size
        } else {
            10
        };
        format!("limit {} of {}", page_size, (page_num - 1) * page_size)
    }

    pub fn condition_and_page() -> Vec<String> {
        let mut query = String::from("SELECT * FROM table_name");
        let mut params = Vec::new();
        let mut param_index = 1;

        if let Some(id) = query_form.id {
            query.push_str(&format!(" WHERE id = ${}", param_index));
            params.push(id.into());
            param_index += 1;
        }

        if let Some(name) = query_form.name {
            if params.is_empty() {
                query.push_str(&format!(" WHERE name = ${}", param_index));
            } else {
                query.push_str(&format!(" AND name = ${}", param_index));
            }
            params.push(name.into());
            param_index += 1;
        }

        if let Some(page_size) = query_form.page_size {
            if let Some(page_num) = query_form.page_num {
                query.push_str(&format!(
                    " LIMIT ${} OFFSET ${}",
                    param_index,
                    param_index + 1
                ));
                params.push(page_size.into());
                params.push((page_num - 1) * page_size);
                param_index += 2;
            }
        }
    }
}
