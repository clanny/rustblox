use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupPermissions {
    pub group_posts_permissions: GroupPostPermissions,
    pub group_membership_permissions: GroupMembershipPermissions,
    pub group_management_permissions: GroupManagementPermissions,
    pub group_economy_permissions: GroupEconomyPermissions,
    pub group_open_cloud_permissions: GroupOpenCloudPermissions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupPostPermissions {
    pub view_wall: bool,
    pub post_to_wall: bool,
    pub delete_from_wall: bool,
    pub view_status: bool,
    pub post_to_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipPermissions {
    pub change_rank: bool,
    pub invite_members: bool,
    pub remove_members: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupManagementPermissions {
    pub manage_relationships: bool,
    pub manage_clan: bool,
    pub view_audit_logs: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupEconomyPermissions {
    pub spend_group_funds: bool,
    pub advertise_group: bool,
    pub create_items: bool,
    pub manage_items: bool,
    pub add_group_places: bool,
    pub manage_group_games: bool,
    pub view_group_payouts: bool,
    pub view_analytics: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupOpenCloudPermissions {
    pub use_cloud_authentication: bool,
    pub administer_cloud_authentication: bool,
}
