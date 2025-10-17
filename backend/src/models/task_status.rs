use async_graphql::Enum;

#[derive(sqlx::Type, Debug, Enum, Copy, Clone, Eq, PartialEq)]
#[sqlx(type_name = "task_status")]
#[graphql(rename_items = "PascalCase")]

pub enum TaskStatus {
    #[sqlx(rename = "Not Started")] // Map each variant to its exact DB string
    NotStarted,
    #[sqlx(rename = "In Progress")]
    InProgress,
    #[sqlx(rename = "Blocked")]
    Blocked,
    #[sqlx(rename = "Under Review")]
    UnderReview,
    #[sqlx(rename = "Deprecated")]
    Deprecated,
    #[sqlx(rename = "Completed")]
    Completed,
}
