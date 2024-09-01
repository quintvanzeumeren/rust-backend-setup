CREATE TABLE "organisations"
(
    "id"   UUID PRIMARY KEY,
    "name" varchar        NOT NULL,
    "slug" varchar UNIQUE NOT NULL
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

CREATE TABLE "permissions"
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

CREATE UNIQUE INDEX ON "permissions" ("role_id", "resource_id", "name");

CREATE UNIQUE INDEX ON "resources" ("type", "resource_specific_id");

ALTER TABLE "user_roles"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

ALTER TABLE "user_roles"
    ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");

ALTER TABLE "roles"
    ADD FOREIGN KEY ("organisation_id") REFERENCES "organisations" ("id");

ALTER TABLE "permissions"
    ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");

ALTER TABLE "permissions"
    ADD FOREIGN KEY ("resource_id") REFERENCES "resources" ("id");
