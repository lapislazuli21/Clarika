use async_graphql::{ID, Object};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct GrowthTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub core_competencies: String,
    pub developing_skills: String,
    pub recent_achievements: String,
    pub how_to_contribute: String,
}

#[Object]
impl GrowthTemplate {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn user_id(&self) -> ID {
        ID(self.user_id.to_string())
    }

    async fn core_competencies(&self) -> &str {
        &self.core_competencies
    }

    async fn developing_skills(&self) -> &str {
        &self.developing_skills
    }

    async fn recent_achievements(&self) -> &str {
        &self.recent_achievements
    }

    async fn how_to_contribute(&self) -> &str {
        &self.how_to_contribute
    }
}
