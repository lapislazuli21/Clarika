use super::{raci_assignment::RaciAssignment, task_status::TaskStatus};
use crate::db::DbPool;
use async_graphql::{Context, ID, Object};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub project_id: Uuid,
    pub assigned_to_id: Option<Uuid>,
    pub status: TaskStatus,
    pub deadline: Option<DateTime<Utc>>,
    pub jira_ticket_id: Option<String>,
}

#[Object]
impl Task {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }
    async fn title(&self) -> &str {
        &self.title
    }
    async fn project_id(&self) -> ID {
        ID(self.project_id.to_string())
    }
    async fn assigned_to_id(&self) -> Option<ID> {
        self.assigned_to_id.map(|id| ID(id.to_string()))
    }
    async fn status(&self) -> TaskStatus {
        self.status
    }
    async fn deadline(&self) -> Option<DateTime<Utc>> {
        self.deadline
    }

    async fn jira_ticket_id(&self) -> Option<&str> {
        self.jira_ticket_id.as_deref()
    }

    async fn raci_assignments(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<RaciAssignment>> {
        let pool = ctx.data::<DbPool>()?;
        let assignments = sqlx::query_as!(
            RaciAssignment,
            r#"
            SELECT user_id, task_id, role AS "role: _"
            FROM raci_assignments WHERE task_id = $1
            "#,
            self.id
        )
        .fetch_all(pool)
        .await?;
        Ok(assignments)
    }
}
