-- Add migration script here
CREATE TABLE workflow_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,

    created_by_id UUID NOT NULL REFERENCES users(id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE workflow_steps (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    template_id UUID NOT NULL REFERENCES workflow_templates(id) ON DELETE CASCADE,

    step_name VARCHAR(255) NOT NULL,

    step_order INT NOT NULL,

    role VARCHAR(100),

    depends_on_step_id UUID REFERENCES workflow_steps(id) ON DELETE SET NULL
);

CREATE INDEX idx_workflow_steps_template_id ON workflow_steps(template_id);