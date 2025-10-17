use super::task::Task;
use crate::db::DbPool;
use async_graphql::{Context, ID, Object};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub owner_id: Uuid,
}

#[Object]
impl Project {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    async fn deadline(&self) -> Option<DateTime<Utc>> {
        self.deadline
    }
    async fn owner_id(&self) -> ID {
        ID(self.owner_id.to_string())
    }
    // Resolver Field
    async fn tasks(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Task>> {
        let pool = ctx.data::<DbPool>()?;
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT id, title, project_id, assigned_to_id, status AS "status: _", deadline, jira_ticket_id
            FROM tasks WHERE project_id = $1
            "#,
            self.id
        )
        .fetch_all(pool)
        .await?;
        Ok(tasks)
    }
}
