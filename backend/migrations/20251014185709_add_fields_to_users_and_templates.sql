-- Add migration script here
ALTER TABLE users
ADD COLUMN role VARCHAR(50) NOT NULL DEFAULT 'Employee';

ALTER TABLE growth_templates
ADD COLUMN quarter VARCHAR(10) NOT NULL DEFAULT 'Q4-2025';
