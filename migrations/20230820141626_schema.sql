-- Add migration script here
ALTER TABLE runs
ADD COLUMN run_datetime TIMESTAMP;
ALTER TABLE runs
ALTER COLUMN run_datetime
SET DEFAULT now();