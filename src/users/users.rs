use crate::{
    users::{PartialUser, User},
    util::jar::RequestJar,
    util::Error,
};

pub async fn user_by_id(jar: &mut RequestJar, user_id: usize) -> Result<User, Box<Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}", user_id);
    let response = jar.get_json::<User>(&url).await?;
    Ok(response)
}

pub async fn whoami(jar: &mut RequestJar) -> Result<PartialUser, Box<Error>> {
    let url = "https://users.roblox.com/v1/users/authenticated";
    let response = jar.get_json::<PartialUser>(&url).await?;
    Ok(response)
}