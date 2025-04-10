-- Add migration script here
INSERT INTO jobs(run_after, job_data, version, is_finished) VALUES (NOW(), '\x03'::bytea, 3, 'f');