-- Add migration script here
CREATE TABLE growth_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    core_competencies TEXT NOT NULL,
    developing_skills TEXT NOT NULL,
    recent_achievements TEXT NOT NULL,
    how_to_contribute TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_growth_templates_user_id ON growth_templates(user_id);