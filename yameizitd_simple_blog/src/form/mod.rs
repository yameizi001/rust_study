pub mod category;

pub use category::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionAndPage {
    pub condition: String,
    pub page: String,
}
