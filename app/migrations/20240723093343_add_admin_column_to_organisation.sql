-- Add migration script here
ALTER TABLE "organisations"
    ADD COLUMN admin BOOLEAN NOT NULL DEFAULT FALSE;