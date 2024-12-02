-- Add migration script here
CREATE TABLE "user" (
    id bigint generated always as identity,
    username text not null,
    password_hash text not null,
    PRIMARY KEY (id)
);
create unique index user_username on "user" (username);

CREATE TABLE "sessions" (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE
);

CREATE INDEX sessions_user ON "sessions" (user_id);

CREATE TABLE session_scopes (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    session_id BIGINT NOT NULL,
    scope BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES "sessions" (id) ON DELETE CASCADE
);

CREATE INDEX session_scopes_session ON "session_scopes" (session_id);
CREATE UNIQUE INDEX session_scopes_session_scope ON "session_scopes" (session_id, scope);