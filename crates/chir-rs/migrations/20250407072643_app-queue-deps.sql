-- Add migration script here
ALTER TABLE jobs ADD COLUMN is_finished BOOLEAN;
UPDATE jobs SET is_finished='f';
ALTER TABLE jobs ALTER COLUMN is_finished SET NOT NULL;

DROP INDEX jobs_priority_run_after_running;
CREATE INDEX jobs_priority_run_after ON jobs (priority DESC, run_after ASC) WHERE NOT is_running AND NOT is_finished;
CREATE INDEX jobs_is_finished ON jobs (is_finished);

CREATE TABLE job_deps (
    job_id BIGINT NOT NULL,
    dependency_job_id BIGINT NOT NULL,
    PRIMARY KEY (job_id, dependency_job_id),
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (dependency_job_id) REFERENCES jobs(id) ON DELETE CASCADE
);
