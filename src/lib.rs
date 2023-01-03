use std::fs;

pub mod users;
pub mod util;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn authenticated_jar() -> util::jar::RequestJar {
        // Read credentials from file
        let contents = fs::read_to_string("roblosecurity.txt").unwrap();
        let roblosecurity = contents.trim().to_string();
        let mut jar = util::jar::RequestJar::new().await;
        jar.set_roblosecurity(roblosecurity).await;
        jar
    }

    async fn unauthenticated_jar() -> util::jar::RequestJar {
        util::jar::RequestJar::new().await
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn whoami() {
        let mut jar = authenticated_jar().await;
        let user = crate::users::users::whoami(&mut jar).await.unwrap();
        println!("{:#?}", user);
    }

    #[tokio::test]
    async fn user_by_id() {
        let mut jar = unauthenticated_jar().await;
        let user = crate::users::users::user_by_id(&mut jar, 375760054)
            .await
            .unwrap();
        println!("{:#?}", user);
    }

    // Display names

    #[tokio::test]
    async fn validate_display_name_fail() {
        let mut jar = unauthenticated_jar().await;
        let display_name =
            crate::users::display_names::validate_display_name(&mut jar, "shit".to_string())
                .await
                .unwrap();

        // Panic if it's valid
        match display_name {
            crate::users::display_names::ValidateDisplayNameResponseEnum::Success(_) => {
                panic!("Display name is valid when it shouldn't be")
            }
            _ => {}
        }
    }

    #[tokio::test]
    async fn validate_display_name_success() {
        let mut jar = unauthenticated_jar().await;
        let display_name =
            crate::users::display_names::validate_display_name(&mut jar, "test".to_string())
                .await
                .unwrap();

        // Panic if it's valid
        match display_name {
            crate::users::display_names::ValidateDisplayNameResponseEnum::Failed(_) => {
                panic!("Display name is invalid when it shouldn't be")
            }
            _ => {}
        }
    }

    #[tokio::test]
    async fn validate_display_name_for_user() {
        let mut jar = authenticated_jar().await;
        let user_id = crate::users::users::whoami(&mut jar).await.unwrap().id;
        let display_name = crate::users::display_names::validate_display_name_for_user(
            &mut jar,
            "test".to_string(),
            user_id,
        )
        .await
        .unwrap();

        // Panic if it's valid
        match display_name {
            crate::users::display_names::ValidateDisplayNameResponseEnum::Failed(_) => {
                panic!("Display name is invalid when it shouldn't be")
            }
            _ => {}
        }
    }
}
