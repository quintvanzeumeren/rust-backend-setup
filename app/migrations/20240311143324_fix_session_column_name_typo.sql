-- Add migration script here
ALTER TABLE "user_sessions" RENAME "ending_toking_id" TO "ending_token_id";
