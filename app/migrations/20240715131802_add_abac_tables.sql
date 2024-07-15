-- Add migration script here
CREATE TABLE "organisations"
(
    "id"   UUID PRIMARY KEY,
    "name" varchar        NOT NULL,
    "slug" varchar UNIQUE NOT NULL
);

CREATE TABLE "organisation_members"
(
    "member_id"       UUID PRIMARY KEY,
    "organisation_id" UUID NOT NULL,
    "user_id"         UUID NOT NULL
);

CREATE TABLE "roles"
(
    "id"              UUID PRIMARY KEY,
    "organisation_id" UUID                NOT NULL,
    "role_name"       VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE "member_roles"
(
    "member_id" UUID,
    "role_id"   UUID,
    PRIMARY KEY ("member_id", "role_id")
);

CREATE TABLE "permissions"
(
    "id"              UUID PRIMARY KEY,
    "resource_id"     uuid,
    "permission_name" VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE "role_permissions"
(
    "permission_id" UUID,
    "role_id"       UUID,
    PRIMARY KEY ("permission_id", "role_id")
);

CREATE TABLE "resources"
(
    "id"                   uuid PRIMARY KEY,
    "resource_type"        varchar NOT NULL,
    "resource_specific_id" uuid    NOT NULL
);

CREATE UNIQUE INDEX ON "organisation_members" ("organisation_id", "user_id");

CREATE UNIQUE INDEX ON "resources" ("resource_type", "resource_specific_id");

ALTER TABLE "organisation_members"
    ADD FOREIGN KEY ("organisation_id") REFERENCES "organisations" ("id");

ALTER TABLE "organisation_members"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

ALTER TABLE "roles"
    ADD FOREIGN KEY ("organisation_id") REFERENCES "organisations" ("id");

ALTER TABLE "member_roles"
    ADD FOREIGN KEY ("member_id") REFERENCES "organisation_members" ("member_id");

ALTER TABLE "member_roles"
    ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");

ALTER TABLE "permissions"
    ADD FOREIGN KEY ("resource_id") REFERENCES "resources" ("id");

ALTER TABLE "role_permissions"
    ADD FOREIGN KEY ("permission_id") REFERENCES "permissions" ("id");

ALTER TABLE "role_permissions"
    ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");
