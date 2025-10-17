-- Add migration script here
ALTER TABLE tasks
ADD COLUMN jira_ticket_id VARCHAR(100);