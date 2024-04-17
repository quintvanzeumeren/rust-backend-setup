-- Add migration script here
ALTER TABLE "refresh_tokens"
ADD COLUMN "user_id" uuid NOT NULL;

ALTER TABLE "refresh_tokens"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");