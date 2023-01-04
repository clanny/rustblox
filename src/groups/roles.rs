use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub rank: usize,
    pub member_count: usize,
}
