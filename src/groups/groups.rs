use specta::Type;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{
    users::{whoami, MinimalGroupUser},
    util::{
        jar::RequestJar,
        paging::{get_page, PageLimit, SortOrder},
        responses::DataWrapper,
        Error,
    },
};

use super::{permissions::GroupPermissions, roles::GroupRole};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub owner: MinimalGroupUser,
    pub shout: Option<GroupShout>,
    pub member_count: Option<i64>,
    pub is_builders_club_only: bool,
    pub public_entry_allowed: bool,
    pub is_locked: Option<bool>,
    pub has_verified_badge: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct MinimalGroup {
    pub id: i64,
    pub name: String,
    pub member_count: i64,
    pub has_verified_badge: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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
pub async fn group_by_id(jar: &RequestJar, group_id: i64) -> Result<Group, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}", group_id);
    let response = jar.get_json::<Group>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupAuditLogEntry {
    pub actor: GroupAuditLogActor,
    pub action_type: GroupAuditLogActionType,
    //description: , // FIXME: ??? It shows an empty object in the docs
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupAuditLogActor {
    pub user: MinimalGroupUser,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Display, EnumString, Type)]
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
    jar: &RequestJar,
    group_id: i64,
    limit: PageLimit,
    user_id: Option<i64>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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
    jar: &RequestJar,
    group_id: i64,
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

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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
pub async fn settings(jar: &RequestJar, group_id: i64) -> Result<GroupSettings, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}/settings", group_id);
    let response = jar.get_json::<GroupSettings>(&url).await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupSettingsUpdateRequest {
    pub is_approval_required: Option<bool>,
    pub are_enemies_allowed: Option<bool>,
    pub are_group_funds_visible: Option<bool>,
    pub are_group_games_visible: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupSettingsUpdateResponse {}

/// Updates a group's settings
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
/// - 23: Insufficient permissions to complete the request.
/// - 31: Service is currently unavailable.
pub async fn update_settings(
    jar: &RequestJar,
    group_id: i64,
    request: GroupSettingsUpdateRequest,
) -> Result<GroupSettingsUpdateResponse, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/{}/settings", group_id);
    let response = jar
        .patch_json::<GroupSettingsUpdateResponse, GroupSettingsUpdateRequest>(&url, request)
        .await?;
    Ok(response)
}

// TODO: Figure out how to send the files to /v1/groups/create and implement it

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupComplianceItem {
    pub can_view_group: bool,
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupComplianceResponse {
    pub groups: Vec<GroupComplianceItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupComplianceRequest {
    pub group_ids: Vec<i64>,
}

/// Gets group policy info used for compliance
/// # Error codes
/// - 1: Too many ids in request.
/// - 2: Ids could not be parsed from request.
pub async fn compliance(
    jar: &RequestJar,
    group_ids: Vec<i64>,
) -> Result<GroupComplianceResponse, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/groups/policies");
    let request = GroupComplianceRequest { group_ids };
    let response = jar
        .post_json::<GroupComplianceResponse, GroupComplianceRequest>(&url, request)
        .await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct NewDescriptionRequest {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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
    jar: &RequestJar,
    group_id: i64,
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

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct NewNameRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
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
    jar: &RequestJar,
    group_id: i64,
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

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembership {
    pub group_id: i64,
    pub is_primary: bool,
    pub is_pending_join: bool,
    pub group_role: Option<GroupMembershipUserRole>,
    pub permissions: GroupPermissions,
    pub are_group_games_visible: bool,
    pub are_group_funds_visible: bool,
    pub are_enemies_allowed: bool,
    pub can_configure: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipUserRole {
    pub user: MinimalGroupUser,
    pub role: GroupRole,
}

/// Gets a user's group membership
///
/// # Error codes
/// - 1: Group is invalid or does not exist.
pub async fn membership(jar: &RequestJar, group_id: i64) -> Result<GroupMembership, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/membership",
        group_id
    );
    let response = jar.get_json::<GroupMembership>(&url).await?;
    Ok(response)
}

/// Gets a list of users in a group
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
pub async fn members(
    jar: &RequestJar,
    group_id: i64,
    limit: PageLimit,
    sort_order: Option<SortOrder>,
) -> Result<Vec<GroupMembershipUserRole>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/users?sortOrder={}",
        group_id,
        sort_order.unwrap_or(SortOrder::Asc).get_sort_order_string()
    );
    //let response = jar.get_json::<GroupRoleResponse>(&url).await?;
    let response = get_page(jar, url.as_str(), limit, None).await?;
    Ok(response)
}

// Note: Joining a group is not implemented and will not be implemented, as it is not needed and requires a captcha.

/// Gets all groups the authenticated user is pending for
/// **This does not list the pend requests for a specific group**
///
/// # Error codes
/// There are no error codes for this endpoint.
pub async fn pending_requests(jar: &RequestJar) -> Result<Vec<Group>, Box<Error>> {
    let url = format!("https://groups.roblox.com/v1/user/groups/pending");
    let response = jar
        .get_json::<DataWrapper<Vec<Group>>>(url.as_str())
        .await?;
    Ok(response.data)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct FriendGroupsGroupItem {
    pub group: Group,
    pub role: GroupRole,
    pub is_primary_group: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct FriendGroupsItem {
    pub user: MinimalGroupUser,
    pub groups: Vec<FriendGroupsGroupItem>,
}

/// Gets all the groups the currently authenticated user's friends are in
///
/// # Error codes
/// - 3: The user is invalid or does not exist.
pub async fn friend_groups(jar: &RequestJar) -> Result<Vec<FriendGroupsItem>, Box<Error>> {
    let user_id = whoami(jar).await?.id;
    let url = format!(
        "https://groups.roblox.com/v1/users/{}/friends/groups/roles",
        user_id
    );
    let response = jar
        .get_json::<DataWrapper<Vec<FriendGroupsItem>>>(url.as_str())
        .await?;
    Ok(response.data)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct UserMembershipsGroupItem {
    pub group: MinimalGroup,
    pub role: GroupRole,
}

/// Gets all the groups the specified user is in.
/// It also includes the role the user is
///
/// # Error codes
/// - 3: The user is invalid or does not exist.
pub async fn user_memberships(
    jar: &RequestJar,
    user_id: i64,
) -> Result<Vec<UserMembershipsGroupItem>, Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v2/users/{}/groups/roles",
        user_id
    );
    let response = jar
        .get_json::<DataWrapper<Vec<UserMembershipsGroupItem>>>(url.as_str())
        .await?;
    Ok(response.data)
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupOwnershipChangeRequest {
    pub user_id: i64,
}

/// Changes the owner of a group
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 3: The user is invalid or does not exist.
/// - 15: User is not a member of the group.
/// - 16: The user does not have the necessary level of premium membership.
/// - 17: You are not authorized to change the owner of this group.
/// - 25: 2-Step Verification is required to make further transactions. Go to Settings > Security to complete 2-Step Verification.
pub async fn change_owner(jar: &RequestJar, group_id: i64, user_id: i64) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/change-owner",
        group_id
    );
    let request = GroupOwnershipChangeRequest { user_id };
    jar.post_json(url.as_str(), &request).await?;
    Ok(())
}

/// Claims the ownership of a group
///
/// # Error codes
/// - 1: The group is invalid or does not exist.
/// - 11: You are not authorized to claim this group.
/// - 12: This group already has an owner.
/// - 13: Too many attempts to claim groups. Please try again later.
/// - 18: The operation is temporarily unavailable. Please try again later.
pub async fn claim_ownership(jar: &RequestJar, group_id: i64) -> Result<(), Box<Error>> {
    let url = format!(
        "https://groups.roblox.com/v1/groups/{}/claim-ownership",
        group_id
    );
    jar.post(url.as_str(), "".to_string()).await?;
    Ok(())
}
