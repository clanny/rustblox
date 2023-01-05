use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::util::{jar::RequestJar, Error};

use super::Group;

/// Batch management of group relationships (enemies and allies)
pub mod relationships {}

/// Management of 1 group relationship at a time  (enemies and allies)
pub mod relationship {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display)]
pub enum RelationshipType {
    #[serde(rename = "Enemies")]
    Enemy,
    #[serde(rename = "Allies")]
    Ally,
    All,
}

impl RelationshipType {
    pub fn get_relationship_type_string(&self) -> String {
        match self {
            RelationshipType::Enemy => "Enemies",
            RelationshipType::Ally => "Allies",
            RelationshipType::All => "All",
        }
        .to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupRelationships {
    pub group_id: usize,
    pub relationship_type: RelationshipType,
    pub total_group_count: usize,
    #[serde(rename = "relatedGroups")]
    pub groups: Vec<Group>,
}

/// Retrieves a list of groups that are enemies, allies or both of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 8: Invalid or missing pagination parameters
#[async_recursion::async_recursion]
pub async fn relationships(
    jar: &mut RequestJar,
    group_id: usize,
    relationship_type: RelationshipType,
) -> Result<GroupRelationships, Box<Error>> {
    if relationship_type == RelationshipType::All {
        let enemies = relationships(jar, group_id, RelationshipType::Enemy).await?;
        let allies = relationships(jar, group_id, RelationshipType::Ally).await?;
        let mut groups = enemies.groups;
        groups.extend(allies.groups);
        return Ok(GroupRelationships {
            group_id,
            relationship_type,
            total_group_count: enemies.total_group_count + allies.total_group_count,
            groups,
        });
    }

    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/relationships/{}?model.startRowIndex=0&model.maxRows=1000",
        group_id, relationship_type.get_relationship_type_string()
    );
    let response = jar.get_json::<GroupRelationships>(&url).await?;
    Ok(response)
}

/// Retrieves a list of groups that are enemies of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 8: Invalid or missing pagination parameters
pub async fn enemies(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<GroupRelationships, Box<Error>> {
    relationships(jar, group_id, RelationshipType::Enemy).await
}

/// Retrieves a list of groups that are allies of the specified group.
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 4: Group relationship type or request type is invalid.
/// - 8: Invalid or missing pagination parameters
pub async fn allies(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<GroupRelationships, Box<Error>> {
    relationships(jar, group_id, RelationshipType::Ally).await
}
