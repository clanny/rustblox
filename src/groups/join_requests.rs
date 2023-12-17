pub mod join_requests {
    use rspc::Type;
    use serde::{Deserialize, Serialize};

    use crate::{
        users::MinimalGroupUser,
        util::{
            jar::RequestJar,
            paging::{get_page, PageLimit},
            Error,
        },
    };

    #[derive(Debug, Serialize, Deserialize, Clone, Type)]
    #[serde(rename_all = "camelCase")]
    pub struct BatchRequest {
        pub user_ids: Vec<i64>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Type)]
    #[serde(rename_all = "camelCase")]
    pub struct JoinRequest {
        pub requestor: MinimalGroupUser,
        pub created: String,
    }

    /// Retrieves join requests for a group.
    ///
    /// # Error codes
    /// - 1: The group is invalid or does not exist.
    /// - 19: You have insufficient permissions for this request.
    #[allow(unused)] // It appears to be a bug in the checker, this kinda fixes it.
    pub async fn get(
        jar: &RequestJar,
        group_id: i64,
        limit: PageLimit,
    ) -> Result<Vec<JoinRequest>, Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/join-requests",
            group_id
        );
        let response = get_page(jar, url.as_str(), limit, None).await?;
        Ok(response)
    }

    /// Accepts a batch of join requests for a group.
    ///
    /// # Error codes
    /// - 1: The group is invalid or does not exist.
    /// - 3: The user is invalid or does not exist.
    /// - 6: You are already in the maximum number of groups.
    /// - 18: The operation is temporarily unavailable. Please try again later.
    /// - 19: You have insufficient permissions for this request.
    /// - 20: The group join request is invalid.
    #[allow(unused)] // It appears to be a bug in the checker, this kinda fixes it.
    pub async fn accept(
        jar: &RequestJar,
        group_id: i64,
        user_ids: Vec<i64>,
    ) -> Result<(), Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/join-requests",
            group_id
        );
        let request = BatchRequest { user_ids };
        let response = jar.post_json::<(), BatchRequest>(&url, request).await?;
        Ok(response)
    }

    /// Declines a batch of join requests for a group.
    ///
    /// # Error codes
    /// - 1: The group is invalid or does not exist.
    /// - 3: The user is invalid or does not exist.
    #[allow(unused)] // It appears to be a bug in the checker, this kinda fixes it.
    pub async fn decline(
        jar: &RequestJar,
        group_id: i64,
        user_ids: Vec<i64>,
    ) -> Result<(), Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/join-requests",
            group_id
        );
        let request = BatchRequest { user_ids };
        let response = jar.delete_json::<(), BatchRequest>(&url, request).await?;
        Ok(response)
    }
}

pub mod join_request {
    use rspc::Type;
    use serde::{Deserialize, Serialize};

    use crate::{
        users::MinimalGroupUser,
        util::{jar::RequestJar, Error},
    };

    #[derive(Debug, Serialize, Deserialize, Clone, Type)]
    #[serde(rename_all = "camelCase")]
    pub struct BatchRequest {
        pub user_ids: Vec<i64>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Type)]
    #[serde(rename_all = "camelCase")]
    pub struct JoinRequest {
        pub requestor: MinimalGroupUser,
        pub created: String,
    }

    /// Retrieves a join request for a group.
    ///
    /// # Error codes
    /// - 1: The group is invalid or does not exist.
    /// - 19: You have insufficient permissions for this request.
    pub async fn get(
        jar: &RequestJar,
        group_id: i64,
        user_id: i64,
    ) -> Result<JoinRequest, Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/join-requests/users/{}",
            group_id, user_id
        );
        let response = jar.get_json(url.as_str()).await?;
        Ok(response)
    }

    /// Accepts a join request for a group.
    ///
    /// # Error codes
    /// - 1: The group is invalid or does not exist.
    /// - 3: The user is invalid or does not exist.
    /// - 6: You are already in the maximum number of groups.
    /// - 18: The operation is temporarily unavailable. Please try again later.
    /// - 19: You have insufficient permissions for this request.
    /// - 20: The group join request is invalid.
    pub async fn accept(jar: &RequestJar, group_id: i64, user_id: i64) -> Result<(), Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/join-requests/users/{}",
            group_id, user_id
        );
        jar.post(&url, "".to_string()).await?;
        Ok(())
    }

    /// Declines a join request for a group.
    ///
    /// # Error codes
    /// - 3: The user is invalid or does not exist.
    /// - 4: You do not have permission to manage this member.
    pub async fn decline(jar: &RequestJar, group_id: i64, user_id: i64) -> Result<(), Box<Error>> {
        let url = format!(
            "https://groups.roblox.com/v1/groups/{}/join-requests/users/{}",
            group_id, user_id
        );
        jar.delete(&url, "".to_string()).await?;
        Ok(())
    }
}
