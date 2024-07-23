-- Add migration script here

BEGIN;

ALTER TABLE "roles"
    DROP CONSTRAINT "roles_organisation_id_fkey",
    DROP COLUMN "organisation_id",
    ADD CONSTRAINT "unique_role_name" UNIQUE ("name");

DROP TABLE "organisations";

COMMIT;
