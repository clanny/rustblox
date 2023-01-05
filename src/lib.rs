use std::fs;

pub mod groups;
pub mod users;
pub mod util;

#[cfg(test)]
mod tests {
    use super::*;

    async fn authenticated_jar() -> util::jar::RequestJar {
        // Read credentials from file
        let contents = fs::read_to_string("roblosecurity.txt").unwrap();
        let roblosecurity = contents.trim().to_string();
        let mut jar = util::jar::RequestJar::new().await;
        jar.set_roblosecurity(roblosecurity).await;

        // If the file proxy.txt exists, use it as a proxy
        if fs::metadata("proxy.txt").is_ok() {
            let contents = fs::read_to_string("proxy.txt").unwrap();
            let proxy = contents.trim().to_string();
            jar.set_proxy(proxy);
        }

        jar
    }

    async fn unauthenticated_jar() -> util::jar::RequestJar {
        let mut jar = util::jar::RequestJar::new().await;

        // If the file proxy.txt exists, use it as a proxy
        if fs::metadata("proxy.txt").is_ok() {
            let contents = fs::read_to_string("proxy.txt").unwrap();
            let proxy = contents.trim().to_string();
            jar.set_proxy(proxy);
        }

        jar
    }

    #[tokio::test]
    async fn whoami() {
        let mut jar = authenticated_jar().await;
        let user = crate::users::whoami(&mut jar).await.unwrap();
        println!("{:#?}", user);
    }

    #[tokio::test]
    async fn user_by_id() {
        let mut jar = unauthenticated_jar().await;
        let user = crate::users::user_by_id(&mut jar, 375760054).await.unwrap();
        println!("{:#?}", user);
    }

    // Display names

    #[tokio::test]
    async fn validate_display_name_fail() {
        let mut jar = unauthenticated_jar().await;
        let display_name = crate::users::validate_display_name(&mut jar, "shit".to_string())
            .await
            .unwrap();

        // Panic if it's valid
        match display_name {
            crate::users::ValidateDisplayNameResponseEnum::Success(_) => {
                panic!("Display name is valid when it shouldn't be")
            }
            _ => {}
        }
    }

    #[tokio::test]
    async fn validate_display_name_success() {
        let mut jar = unauthenticated_jar().await;
        let display_name = crate::users::validate_display_name(&mut jar, "test".to_string())
            .await
            .unwrap();

        // Panic if it's valid
        match display_name {
            crate::users::ValidateDisplayNameResponseEnum::Failed(_) => {
                panic!("Display name is invalid when it shouldn't be")
            }
            _ => {}
        }
    }

    #[tokio::test]
    async fn validate_display_name_for_user() {
        let mut jar = authenticated_jar().await;
        let user_id = crate::users::whoami(&mut jar).await.unwrap().id;

        if user_id != 4205503041 {
            return; // This account gets used for CI, we only want to run this test on that account bc ratelimits
        }

        let display_name =
            crate::users::validate_display_name_for_user(&mut jar, "test".to_string(), user_id)
                .await
                .unwrap();

        // Panic if it's valid
        match display_name {
            crate::users::ValidateDisplayNameResponseEnum::Failed(_) => {
                panic!("Display name is invalid when it shouldn't be")
            }
            _ => {}
        }
    }

    //#[tokio::test]
    //async fn set_display_name() {
    //    let mut jar = authenticated_jar().await;
    //    crate::users::set_display_name(&mut jar, "notest".to_string())
    //        .await
    //        .unwrap();
    //    let whoami1 = crate::users::whoami(&mut jar).await.unwrap();
    //    crate::users::set_display_name(&mut jar, "test".to_string())
    //        .await
    //        .unwrap();
    //
    //    let whoami2 = crate::users::whoami(&mut jar).await.unwrap();
    //
    //    assert_ne!(whoami1.display_name, whoami2.display_name);
    //    assert_eq!(whoami2.display_name, "test".to_string());
    //}
    // This test cant work as the display name can only be changed once every 7 days!

    #[tokio::test]
    async fn get_age_bracket() {
        let mut jar = authenticated_jar().await;
        let age_bracket = crate::users::age_bracket(&mut jar).await.unwrap();
        assert_eq!(age_bracket.age_bracket, 0);
    }

    #[tokio::test]
    async fn get_country_code() {
        let mut jar = authenticated_jar().await;
        let country_code = crate::users::country_code(&mut jar).await.unwrap();
        assert_eq!(country_code.country_code, "NL");
    }

    #[tokio::test]
    async fn get_roles() {
        let mut jar = authenticated_jar().await;
        let roles = crate::users::roles(&mut jar).await.unwrap();
        let empty_vec: Vec<String> = Vec::new();
        assert_eq!(roles.roles, empty_vec);
    }

    #[tokio::test]
    async fn bulk_users_by_username() {
        let mut jar = authenticated_jar().await;
        let users = crate::users::bulk_users_by_username(
            &mut jar,
            vec!["piano1029".to_string(), "ClannyBot".to_string()],
        )
        .await
        .unwrap();

        assert_eq!(users.data.len(), 2);
        assert_eq!(users.data[0].name, "piano1029".to_string());
        assert_eq!(users.data[0].id, 375760054);
    }

    #[tokio::test]
    async fn bulk_users_by_id() {
        let mut jar = authenticated_jar().await;
        let users = crate::users::bulk_users_by_id(&mut jar, vec![375760054, 1444131924])
            .await
            .unwrap();

        println!("{:#?}", users);

        assert_eq!(users.data.len(), 2);
        assert_eq!(users.data[0].name, "piano1029".to_string());
    }

    #[tokio::test]
    async fn username_history() {
        let mut jar = unauthenticated_jar().await;
        let users = crate::users::username_history(&mut jar, 375760054)
            .await
            .unwrap();

        println!("{:#?}", users);

        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn username_search() {
        let mut jar = unauthenticated_jar().await;
        let users = crate::users::username_search(
            &mut jar,
            "miemper".to_string(),
            util::paging::PageLimit::Limit10,
        )
        .await;

        match users {
            Ok(users) => {
                assert_eq!(users.len(), 10);
                assert_eq!(users[0].name, "miemper".to_string());
            }
            Err(e) => match *e {
                crate::util::Error::RateLimited => {
                    // This is fine, it just means we're being rate limited
                }
                _ => {
                    // This isn't
                    panic!("Unexpected error: {:?}", e);
                }
            },
        }
    }

    #[tokio::test]
    async fn get_group() {
        let mut jar = unauthenticated_jar().await;
        let group = crate::groups::group_by_id(&mut jar, 7370273).await.unwrap();

        println!("{:#?}", group);

        assert_eq!(group.id, 7370273);
        assert_eq!(group.name, "Clanny Systems".to_string());
        assert_eq!(group.owner.username, "ClannyBot".to_string())
    }

    // TODO: Create test for audit log, but that requires a group (which requires robux)

    #[tokio::test]
    async fn get_group_name_history() {
        let mut jar = unauthenticated_jar().await;
        let group_name_history = crate::groups::name_history(
            &mut jar,
            7370273,
            util::paging::PageLimit::All,
            Some(util::paging::SortOrder::Asc),
        )
        .await
        .unwrap();

        println!("{:#?}", group_name_history);

        assert_eq!(group_name_history.len(), 0);
    }

    // TODO: Re-enable this test when we got a test group
    //#[tokio::test]
    //async fn get_group_settings() {
    //    let mut jar = unauthenticated_jar().await;
    //    let group_settings = crate::groups::settings(&mut jar, 7370273)
    //        .await
    //        .unwrap();
    //
    //    println!("{:#?}", group_settings);
    //
    //    assert_eq!(group_settings.is_approval_required, false)
    //}

    // TODO: Create test for updating group settings, but that requires a group (which requires robux)

    #[tokio::test]
    async fn group_configuration_metadata() {
        let mut jar = unauthenticated_jar().await;
        let group_configuration_metadata = crate::groups::config_metadata(&mut jar).await.unwrap();

        println!("{:#?}", group_configuration_metadata);

        assert_eq!(group_configuration_metadata.role_configuration.min_rank, 0);
    }

    #[tokio::test]
    async fn group_metadata() {
        let mut jar = unauthenticated_jar().await;
        let group_metadata = crate::groups::metadata(&mut jar).await.unwrap();

        println!("{:#?}", group_metadata);

        assert_eq!(group_metadata.show_previous_group_names, true);
    }

    #[tokio::test]
    async fn group_compliance() {
        let mut jar = authenticated_jar().await;
        let group_compliance = crate::groups::compliance(&mut jar, vec![7370273])
            .await
            .unwrap();

        println!("{:#?}", group_compliance);

        assert_eq!(group_compliance.groups.len(), 1);
        assert_eq!(group_compliance.groups[0].can_view_group, true);
        assert_eq!(group_compliance.groups[0].group_id, 7370273);
    }

    // TODO: Create test for updating group description, but that requires a group (which requires robux)
    // We will not be having a test for updating group name cus that requires robux

    // TODO: Create tests for join requests, but that requires a group (which requires robux)

    #[tokio::test]
    async fn group_membership() {
        let mut jar = authenticated_jar().await;
        let group_membership = crate::groups::membership(&mut jar, 7370273).await.unwrap();

        println!("{:#?}", group_membership);

        assert_eq!(group_membership.group_id, 7370273);
        assert_eq!(group_membership.are_group_funds_visible, false);
        assert_eq!(group_membership.can_configure, false);
    }

    #[tokio::test]
    async fn group_roles() {
        let mut jar = authenticated_jar().await;
        let group_roles = crate::groups::roles(&mut jar, 7370273).await.unwrap();

        println!("{:#?}", group_roles);

        assert_eq!(group_roles.len(), 10);
        assert_eq!(group_roles[0].name, "Guest".to_string());
        assert_eq!(
            group_roles[group_roles.len() - 1].name,
            "Clanny".to_string()
        );
    }

    #[tokio::test]
    async fn group_role_members() {
        let mut jar = authenticated_jar().await;
        let group_roles = crate::groups::roles(&mut jar, 7370273).await.unwrap();

        let clanny_role_id = group_roles[group_roles.len() - 1].id;

        let group_role_members = crate::groups::users_on_role(
            &mut jar,
            7370273,
            clanny_role_id,
            util::paging::PageLimit::Limit10,
            None,
        )
        .await
        .unwrap();

        println!("{:#?}", group_role_members);

        assert_eq!(group_role_members.len(), 1);
        assert_eq!(group_role_members[0].username, "ClannyBot".to_string());
    }
}
