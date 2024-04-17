-- Add migration script here
CREATE TABLE IF NOT EXISTS "user_sessions"
(
    "id"                  UUID PRIMARY KEY,
    "user_id"             UUID                   NOT NULL,
    "created_at"          timestamp              NOT NULL,
    "ended_at"            timestamp              NULL,
    "ending_reason"       varchar(64)            NULL,
    "ending_toking_id"    UUID                   NULL
);

CREATE TABLE IF NOT EXISTS "refresh_tokens"
(
    "id"              UUID PRIMARY KEY NOT NULL,
    "parent_id"       UUID             NULL,
    "session_id"      UUID             NOT NULL,
    "issued_at"       timestamp        NOT NULL,
    "not_before"      timestamp        NOT NULL,
    "expiration"      timestamp        NOT NULL,
    "used_at"         timestamp        NULL
);

ALTER TABLE "refresh_tokens"
    ADD FOREIGN KEY ("session_id") REFERENCES "user_sessions" ("id");

ALTER TABLE "refresh_tokens"
    ADD FOREIGN KEY ("parent_id") REFERENCES "refresh_tokens" ("id");

ALTER TABLE "user_sessions"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

ALTER TABLE "user_sessions"
    ADD FOREIGN KEY ("ending_toking_id") REFERENCES "refresh_tokens" ("id");
