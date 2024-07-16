BEGIN;

-- Remove unique on just role_name column
ALTER TABLE "roles"
    DROP CONSTRAINT "roles_role_name_key";

-- Rename role_name to name
ALTER TABLE "roles"
    RENAME COLUMN "role_name" TO "name";

-- Add the new constraint for organisation and name
CREATE UNIQUE INDEX ON "roles" ("organisation_id", "name");

COMMIT;