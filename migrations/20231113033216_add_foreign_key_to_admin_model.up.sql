-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE 
    admins
ADD
    COLUMN auth_user_id UUID 
    REFERENCES auth_users(id)
    ON DELETE CASCADE;