-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

INSERT INTO "roles" (id, name)
values (uuid_generate_v4(), 'root');