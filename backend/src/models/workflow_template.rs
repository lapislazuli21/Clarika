use super::workflow_step::WorkflowStep;
use crate::db::DbPool;
use async_graphql::{Context, ID, Object, Result};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct WorkflowTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by_id: Uuid,
}

#[Object]
impl WorkflowTemplate {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    // This nested resolver fetches all steps for this template, ordered correctly.
    async fn steps(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowStep>> {
        let pool = ctx.data::<DbPool>()?;
        let steps = sqlx::query_as!(
            WorkflowStep,
            "SELECT * FROM workflow_steps WHERE template_id = $1 ORDER BY step_order ASC",
            self.id
        )
        .fetch_all(pool)
        .await?;
        Ok(steps)
    }
}
