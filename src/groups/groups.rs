use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{
    users::users::MinimalGroupUser,
    util::{
        jar::RequestJar,
        paging::{get_page, PageLimit, SortOrder},
        Error,
    },
};

use super::{permissions::GroupPermissions, roles::GroupRole};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub owner: MinimalGroupUser,
    pub shout: GroupShout,
    pub member_count: usize,
    pub is_builders_club_only: bool,
    pub public_entry_allowed: bool,
    pub has_verified_badge: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupShout {
    pub body: String,
    pub poster: MinimalGroupUser,
    pub created: String,
    pub updated: String,
}

/// Gets a group by its group ID
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
pub async fn group_by_id(jar: &mut RequestJar, group_id: usize) -> Result<Group, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}", group_id);
    let response = jar.get_json::<Group>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupAuditLogEntry {
    pub actor: GroupAuditLogActor,
    pub action_type: GroupAuditLogActionType,
    //description: , // FIXME: ??? It shows an empty object in the docs
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupAuditLogActor {
    pub user: MinimalGroupUser,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display, EnumString)]
pub enum GroupAuditLogActionType {
    // This is imported from the docs's HTML
    DeletePost,
    RemoveMember,
    AcceptJoinRequest,
    DeclineJoinRequest,
    PostStatus,
    ChangeRank,
    BuyAd,
    SendAllyRequest,
    CreateEnemy,
    AcceptAllyRequest,
    DeclineAllyRequest,
    DeleteAlly,
    DeleteEnemy,
    AddGroupPlace,
    RemoveGroupPlace,
    CreateItems,
    ConfigureItems,
    SpendGroupFunds,
    ChangeOwner,
    Delete,
    AdjustCurrencyAmounts,
    Abandon,
    Claim,
    Rename,
    ChangeDescription,
    InviteToClan,
    KickFromClan,
    CancelClanInvite,
    BuyClan,
    CreateGroupAsset,
    UpdateGroupAsset,
    ConfigureGroupAsset,
    RevertGroupAsset,
    CreateGroupDeveloperProduct,
    ConfigureGroupGame,
    Lock,
    Unlock,
    CreateGamePass,
    CreateBadge,
    ConfigureBadge,
    SavePlace,
    PublishPlace,
    UpdateRolesetRank,
    UpdateRolesetData,
}

/// Gets the audit log for a group
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 23: Insufficient permissions to complete the request.
pub async fn audit_log(
    jar: &mut RequestJar,
    group_id: usize,
    limit: PageLimit,
    user_id: Option<usize>,
    sort_order: Option<SortOrder>,
    //cursor: Option<String>,
) -> Result<Vec<GroupAuditLogEntry>, Box<Error>> {
    let mut url = format!("https://groups.roblox.com/v1/groups/{}/audit-log", group_id);
    if user_id.is_some() {
        let user_id = user_id.unwrap();
        url = format!("{}?userId={}", url, user_id);
    }
    url = format!("{}&sortOrder={}", url, sort_order.unwrap_or(SortOrder::Asc));

    let response = get_page(jar, url.as_str(), limit, None).await?; // TODO: cursor
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupNameHistoryEntry {
    pub name: String,
    pub created: String,
}

/// Gets the name history for a group
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 23: Insufficient permissions to complete the request.
pub async fn name_history(
    jar: &mut RequestJar,
    group_id: usize,
    limit: PageLimit,
    sort_order: Option<SortOrder>,
    //cursor: Option<String>,
) -> Result<Vec<GroupNameHistoryEntry>, Box<Error>> {
    let mut url = format!(
        "https://groups.roblox.com/v1/groups/{}/name-history",
        group_id
    );
    url = format!("{}?sortOrder={}", url, sort_order.unwrap_or(SortOrder::Asc));

    let response = get_page(jar, url.as_str(), limit, None).await?; // TODO: cursor
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSettings {
    pub is_approval_required: bool,
    pub is_builders_club_required: bool,
    pub are_enemies_allowed: bool,
    pub are_group_funds_visible: bool,
    pub are_group_games_visible: bool,
    pub is_group_name_change_enabled: bool,
}

/// Gets a group's settings
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 23: Insufficient permissions to complete the request.
pub async fn settings(jar: &mut RequestJar, group_id: usize) -> Result<GroupSettings, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}/settings", group_id);
    let response = jar.get_json::<GroupSettings>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSettingsUpdateRequest {
    pub is_approval_required: Option<bool>,
    pub are_enemies_allowed: Option<bool>,
    pub are_group_funds_visible: Option<bool>,
    pub are_group_games_visible: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSettingsUpdateResponse {}

/// Updates a group's settings
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 23: Insufficient permissions to complete the request.
/// - 31: Service is currently unavailable.
pub async fn update_settings(
    jar: &mut RequestJar,
    group_id: usize,
    request: GroupSettingsUpdateRequest,
) -> Result<GroupSettingsUpdateResponse, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}/settings", group_id);
    let response = jar
        .patch_json::<GroupSettingsUpdateResponse, GroupSettingsUpdateRequest>(&url, request)
        .await?;
    Ok(response)
}

// TODO: Figure out how to send the files to /v1/groups/create and implement it

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupComplianceItem {
    pub can_view_group: bool,
    pub group_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupComplianceResponse {
    pub groups: Vec<GroupComplianceItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupComplianceRequest {
    pub group_ids: Vec<usize>,
}

/// Gets group policy info used for compliance
/// # Error codes
/// - 1: Too many ids in request.
/// - 2: Ids could not be parsed from request.
pub async fn compliance(
    jar: &mut RequestJar,
    group_ids: Vec<usize>,
) -> Result<GroupComplianceResponse, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/policies");
    let request = GroupComplianceRequest { group_ids };
    let response = jar
        .post_json::<GroupComplianceResponse, GroupComplianceRequest>(&url, request)
        .await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDescriptionRequest {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDescriptionResponse {
    pub new_description: String,
}

/// Updates a group's description
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 18: The description is too long.
/// - 23: Insufficient permissions to complete the request.
/// - 29: Your group description was empty.
pub async fn update_description(
    jar: &mut RequestJar,
    group_id: usize,
    description: String,
) -> Result<NewDescriptionResponse, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/description",
        group_id
    );
    let request = NewDescriptionRequest { description };
    let response = jar
        .patch_json::<NewDescriptionResponse, NewDescriptionRequest>(&url, request)
        .await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNameRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNameResponse {
    pub new_name: String,
}

/// Updates a group's name
///
/// **THIS COSTS ROBUX!**
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 18: The description is too long.
/// - 23: Insufficient permissions to complete the request.
/// - 29: Your group description was empty.
pub async fn update_name(
    jar: &mut RequestJar,
    group_id: usize,
    name: String,
) -> Result<NewNameResponse, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/description",
        group_id
    );
    let request = NewNameRequest { name };
    let response = jar
        .patch_json::<NewNameResponse, NewNameRequest>(&url, request)
        .await?;
    Ok(response)
}

// FIXME: There is an endpoint PATCH /v1/groups/{groupId}/status, is it needed? what does it do? pls research

// TODO: Implement /v1/groups/icon, i have no idea how to upload files

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembership {
    pub group_id: usize,
    pub is_primary: bool,
    pub is_pending_join: bool,
    pub group_role: Option<GroupMembershipUserRole>,
    pub permissions: GroupPermissions,
    pub are_group_games_visible: bool,
    pub are_group_funds_visible: bool,
    pub are_enemies_allowed: bool,
    pub can_configure: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipUserRole {
    pub user: MinimalGroupUser,
    pub role: GroupRole,
}

/// Gets a user's group membership
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
pub async fn membership(
    jar: &mut RequestJar,
    group_id: usize,
) -> Result<GroupMembership, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/membership",
        group_id
    );
    let response = jar.get_json::<GroupMembership>(&url).await?;
    Ok(response)
}
