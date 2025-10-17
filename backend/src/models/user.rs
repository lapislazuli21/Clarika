use async_graphql::{ID, Object};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

#[Object]
impl User {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    // This function becomes the "email" field.
    async fn email(&self) -> &str {
        &self.email
    }
}
