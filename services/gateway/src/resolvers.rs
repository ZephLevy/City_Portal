use crate::models::User;
use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, world!"
    }

    async fn testing(&self) -> &str {
        "Testing"
    }

    async fn user(&self, id: i32) -> User {
        User {
            id,
            name: String::from("Zeph"),
        }
    }
}
