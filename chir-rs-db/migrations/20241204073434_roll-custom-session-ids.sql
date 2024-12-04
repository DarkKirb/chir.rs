-- Add migration script here
drop index sessions_user;
drop index session_scopes_session;
drop index session_scopes_session_scope;
drop table "session_scopes";
drop table "sessions";

CREATE TABLE "sessions" (
    id BYTEA NOT NULL,
    user_id BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE
);

CREATE INDEX sessions_user ON "sessions" (user_id);

CREATE TABLE session_scopes (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    session_id BYTEA NOT NULL,
    scope BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES "sessions" (id) ON DELETE CASCADE
);

CREATE INDEX session_scopes_session ON "session_scopes" (session_id);
CREATE UNIQUE INDEX session_scopes_session_scope ON "session_scopes" (session_id, scope);