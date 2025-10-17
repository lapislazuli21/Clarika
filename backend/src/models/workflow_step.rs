use async_graphql::{ID, Object};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct WorkflowStep {
    pub id: Uuid,
    pub template_id: Uuid,
    pub step_name: String,
    pub step_order: i32,
    pub role: Option<String>,
    pub depends_on_step_id: Option<Uuid>,
}

#[Object]
impl WorkflowStep {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }
    async fn step_name(&self) -> &str {
        &self.step_name
    }
    async fn step_order(&self) -> i32 {
        self.step_order
    }
    async fn role(&self) -> Option<&str> {
        self.role.as_deref()
    }
    async fn depends_on_step_id(&self) -> Option<ID> {
        self.depends_on_step_id.map(|id| ID(id.to_string()))
    }
}
