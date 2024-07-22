CREATE TABLE "organisations"
(
    "id"   UUID PRIMARY KEY,
    "name" varchar        NOT NULL,
    "slug" varchar UNIQUE NOT NULL
);

CREATE TABLE "users"
(
    "id" UUID PRIMARY KEY
);

CREATE TABLE "user_roles"
(
    "user_id" UUID NOT NULL,
    "role_id" UUID NOT NULL,
    PRIMARY KEY ("user_id", "role_id")
);

CREATE TABLE "roles"
(
    "id"              UUID PRIMARY KEY,
    "name"            varchar NOT NULL,
    "organisation_id" UUID    NOT NULL
);

CREATE TABLE "Permissions"
(
    "id"          UUID PRIMARY KEY,
    "role_id"     UUID    NOT NULL,
    "resource_id" UUID    NOT NULL,
    "name"        varchar NOT NULL
);

CREATE TABLE "resources"
(
    "id"                   UUID PRIMARY KEY,
    "type"                 varchar NOT NULL,
    "resource_specific_id" UUID    NOT NULL
);

CREATE UNIQUE INDEX ON "roles" ("name", "organisation_id");

CREATE UNIQUE INDEX ON "Permissions" ("role_id", "resource_id", "name");

CREATE UNIQUE INDEX ON "resources" ("type", "resource_specific_id");

ALTER TABLE "user_roles"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "user_roles"
    ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");

ALTER TABLE "roles"
    ADD FOREIGN KEY ("organisation_id") REFERENCES "organisations" ("id");

ALTER TABLE "Permissions"
    ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");

ALTER TABLE "Permissions"
    ADD FOREIGN KEY ("resource_id") REFERENCES "resources" ("id");
