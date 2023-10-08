-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE 
    auth_users
ADD
    COLUMN access_token VARCHAR(255);