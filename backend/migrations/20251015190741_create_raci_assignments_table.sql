-- Add migration script here
CREATE TYPE raci_role AS ENUM ('Responsible', 'Accountable', 'Consulted', 'Informed');

CREATE TABLE raci_assignments (
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role raci_role NOT NULL,

    PRIMARY KEY (task_id, user_id)
);

CREATE INDEX idx_raci_assignments_task_id ON raci_assignments(task_id);
CREATE INDEX idx_raci_assignments_user_id ON raci_assignments(user_id);