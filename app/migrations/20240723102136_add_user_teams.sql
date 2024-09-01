-- Add migration script here
CREATE TABLE "teams" (
    "id" UUID PRIMARY KEY
);

CREATE TABLE "team_members" (
    "team_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    PRIMARY KEY ("team_id", "user_id")
);

ALTER TABLE "team_members" ADD FOREIGN KEY ("team_id") REFERENCES "teams" ("id");

ALTER TABLE "team_members" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");
