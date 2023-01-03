// This is used for testing

mod users;
mod util;

#[tokio::main]
async fn main() {
    let mut jar = util::jar::RequestJar::new().await;
    users::users::user_by_id(&mut jar, 375760054).await.unwrap();
}
