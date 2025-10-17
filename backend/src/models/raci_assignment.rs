use super::{raci_role::RaciRole, user::User};
use crate::db::DbPool;
use async_graphql::{Context, Object, Result};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct RaciAssignment {
    pub user_id: Uuid,
    pub task_id: Uuid,
    pub role: RaciRole,
}

#[Object]
impl RaciAssignment {
    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let pool = ctx.data::<DbPool>()?;
        let user = sqlx::query_as!(
            User,
            "SELECT id, email FROM users WHERE id = $1",
            self.user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    async fn role(&self) -> RaciRole {
        self.role
    }
}
