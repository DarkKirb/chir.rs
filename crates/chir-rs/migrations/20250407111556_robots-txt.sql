-- Add migration script here

CREATE TABLE robots(
    robot_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_agent TEXT NOT NULL,
    robot_path TEXT NOT NULL,
    allow BOOLEAN NOT NULL,
    UNIQUE (user_agent, robot_path)
);

