use crate::{
    ai::project_scoper::scope_project,
    db::DbPool,
    models::{
        growth_template::GrowthTemplate, project::Project, raci_assignment::RaciAssignment,
        raci_role::RaciRole, task::Task, task_status::TaskStatus, user::User,
        workflow_step::WorkflowStep, workflow_template::WorkflowTemplate,
    },
};
use async_graphql::{Context, EmptySubscription, ID, Object, Schema};
use uuid::Uuid;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health(&self) -> &'static str {
        "Server is up and running!"
    }

    async fn get_projects(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Project>> {
        let pool = ctx.data::<DbPool>()?;

        // TODO: Replace with the actual logged-in user's ID
        let owner_id = Uuid::parse_str("dfbdcf5a-42b0-4814-825e-86e9b1476575").unwrap();

        let projects = sqlx::query_as!(
            Project,
            "SELECT id, name, description, deadline, owner_id FROM projects WHERE owner_id = $1",
            owner_id
        )
        .fetch_all(pool)
        .await?;

        Ok(projects)
    }

    async fn get_project_by_id(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> async_graphql::Result<Option<Project>> {
        let pool = ctx.data::<DbPool>()?;
        let project_id = Uuid::parse_str(&id)?;

        let project = sqlx::query_as!(
            Project,
            "SELECT id, name, description, deadline, owner_id FROM projects WHERE id = $1",
            project_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(project)
    }

    async fn get_users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let pool = ctx.data::<DbPool>()?;
        let users = sqlx::query_as!(User, "SELECT id, email FROM users ORDER BY email")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    async fn get_workflow_templates(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<WorkflowTemplate>> {
        let pool = ctx.data::<DbPool>()?;
        let templates = sqlx::query_as!(
            WorkflowTemplate,
            "SELECT id, name, description, created_by_id FROM workflow_templates ORDER BY name"
        )
        .fetch_all(pool)
        .await?;
        Ok(templates)
    }

    async fn get_workflow_template_by_id(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> async_graphql::Result<Option<WorkflowTemplate>> {
        let pool = ctx.data::<DbPool>()?;
        let template_id = Uuid::parse_str(&id)?;

        let template = sqlx::query_as!(
            WorkflowTemplate,
            "SELECT id, name, description, created_by_id FROM workflow_templates WHERE id = $1",
            template_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(template)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> async_graphql::Result<User> {
        let pool = ctx.data::<DbPool>()?;
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        let new_user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email",
            email.to_lowercase(),
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(new_user)
    }

    async fn create_growth_template(
        &self,
        ctx: &Context<'_>,
        core_competencies: String,
        developing_skills: String,
        recent_achievements: String,
        how_to_contribute: String,
    ) -> async_graphql::Result<GrowthTemplate> {
        let pool = ctx.data::<DbPool>()?;

        // TODO: Once user login is implemented, get the user_id from the authenticated session token.
        let user_id = Uuid::parse_str("dfbdcf5a-42b0-4814-825e-86e9b1476575").unwrap();

        let new_template = sqlx::query_as!(
            GrowthTemplate,
            r#"INSERT INTO growth_templates (user_id, core_competencies, developing_skills, recent_achievements, how_to_contribute) VALUES ($1, $2, $3, $4, $5) RETURNING id, user_id, core_competencies, developing_skills, recent_achievements, how_to_contribute"#,
            user_id,
            core_competencies,
            developing_skills,
            recent_achievements,
            how_to_contribute
        )
        .fetch_one(pool)
        .await?;

        Ok(new_template)
    }

    async fn scope_project_with_ai(
        &self,
        project_description: String,
    ) -> async_graphql::Result<String> {
        match scope_project(project_description).await {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(async_graphql::Error::new(format!(
                "led to scope project with AI: {}",
                e
            ))),
        }
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
    ) -> async_graphql::Result<Project> {
        let pool = ctx.data::<DbPool>()?;

        // TODO: Replace with the actual logged-in user's ID
        let owner_id = Uuid::parse_str("dfbdcf5a-42b0-4814-825e-86e9b1476575").unwrap();

        let new_project = sqlx::query_as!(
            Project,
            "INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3)
            RETURNING id, name, description, deadline, owner_id",
            name,
            description,
            owner_id
        )
        .fetch_one(pool)
        .await?;

        Ok(new_project)
    }

    async fn create_task(
        &self,
        ctx: &Context<'_>,
        title: String,
        project_id: ID,
    ) -> async_graphql::Result<Task> {
        let pool = ctx.data::<DbPool>()?;
        let project_uuid = Uuid::parse_str(&project_id)?;

        let new_task = sqlx::query_as!(
            Task,
            "INSERT INTO tasks (title, project_id) VALUES ($1, $2)
            RETURNING id, title, project_id, assigned_to_id, status AS \"status: _\", deadline, jira_ticket_id",
            title,
            project_uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(new_task)
    }

    async fn update_task_status(
        &self,
        ctx: &Context<'_>,
        task_id: ID,
        status: TaskStatus,
    ) -> async_graphql::Result<Task> {
        let pool = ctx.data::<DbPool>()?;
        let task_uuid = Uuid::parse_str(&task_id)?;

        let updated_task = sqlx::query_as!(
            Task,
            r#"
            UPDATE tasks SET status = $1 WHERE id = $2
            RETURNING id, title, project_id, assigned_to_id, status AS "status: _", deadline, jira_ticket_id
            "#,
            status as _, // Cast the Rust enum to the DB enum type
            task_uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_task)
    }

    async fn assign_raci_role(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        task_id: ID,
        role: RaciRole,
    ) -> async_graphql::Result<RaciAssignment> {
        let pool = ctx.data::<DbPool>()?;
        let user_uuid = Uuid::parse_str(&user_id)?;
        let task_uuid = Uuid::parse_str(&task_id)?;

        // This SQL performs an "upsert"
        let assignment = sqlx::query_as!(
            RaciAssignment,
            r#"
            INSERT INTO raci_assignments (user_id, task_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, task_id) DO UPDATE SET role = EXCLUDED.role
            RETURNING user_id, task_id, role AS "role: _"
            "#,
            user_uuid,
            task_uuid,
            role as _
        )
        .fetch_one(pool)
        .await?;

        Ok(assignment)
    }

    async fn create_workflow_template(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
    ) -> async_graphql::Result<WorkflowTemplate> {
        let pool = ctx.data::<DbPool>()?;
        // TODO: Replace with the actual logged-in user's ID
        let created_by_id = Uuid::parse_str("dfbdcf5a-42b0-4814-825e-86e9b1476575").unwrap();

        let template = sqlx::query_as!(
            WorkflowTemplate,
            "INSERT INTO workflow_templates (name, description, created_by_id) VALUES ($1, $2, $3)
            RETURNING id, name, description, created_by_id",
            name,
            description,
            created_by_id
        )
        .fetch_one(pool)
        .await?;
        Ok(template)
    }

    async fn add_workflow_step(
        &self,
        ctx: &Context<'_>,
        template_id: ID,
        step_name: String,
        step_order: i32,
        role: Option<String>,
    ) -> async_graphql::Result<WorkflowStep> {
        let pool = ctx.data::<DbPool>()?;
        let template_uuid = Uuid::parse_str(&template_id)?;

        let step = sqlx::query_as!(
            WorkflowStep,
            "INSERT INTO workflow_steps (template_id, step_name, step_order, role) VALUES ($1, $2, $3, $4)
            RETURNING *",
            template_uuid,
            step_name,
            step_order,
            role,
        )
        .fetch_one(pool)
        .await?;
        Ok(step)
    }

    async fn apply_workflow_template_to_project(
        &self,
        ctx: &Context<'_>,
        template_id: ID,
        project_id: ID,
    ) -> async_graphql::Result<Vec<Task>> {
        let pool = ctx.data::<DbPool>()?;
        let template_uuid = Uuid::parse_str(&template_id)?;
        let project_uuid = Uuid::parse_str(&project_id)?;

        // 1. Fetch all steps for the template, in order.
        let steps = sqlx::query_as!(
            WorkflowStep,
            "SELECT * FROM workflow_steps WHERE template_id = $1 ORDER BY step_order ASC",
            template_uuid
        )
        .fetch_all(pool)
        .await?;

        // 2. Create a new task for each step.
        let mut new_tasks = Vec::new();
        for step in steps {
            let new_task = sqlx::query_as!(
                Task,
                r#"
                INSERT INTO tasks (title, project_id) VALUES ($1, $2)
                RETURNING id, title, project_id, assigned_to_id, status AS "status: _", deadline, jira_ticket_id
                "#,
                step.step_name,
                project_uuid
            )
            .fetch_one(pool)
            .await?;
            new_tasks.push(new_task);
        }

        // 3. Return the newly created tasks.
        Ok(new_tasks)
    }

    async fn link_jira_ticket(
        &self,
        ctx: &Context<'_>,
        task_id: ID,
        jira_ticket_id: String,
    ) -> async_graphql::Result<Task> {
        let pool = ctx.data::<DbPool>()?;
        let task_uuid = Uuid::parse_str(&task_id)?;

        let updated_task = sqlx::query_as!(
            Task,
            r#"
            UPDATE tasks SET jira_ticket_id = $1 WHERE id = $2
            RETURNING id, title, project_id, assigned_to_id, status AS "status: _", deadline, jira_ticket_id
            "#,
            jira_ticket_id,
            task_uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_task)
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
