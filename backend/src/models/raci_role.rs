use async_graphql::Enum;

#[derive(sqlx::Type, Debug, Enum, Copy, Clone, Eq, PartialEq)]
#[sqlx(type_name = "raci_role", rename_all = "PascalCase")]
#[graphql(rename_items = "PascalCase")]
pub enum RaciRole {
    Responsible,
    Accountable,
    Consulted,
    Informed,
}
