-- Add up migration script here


CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS customers (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        identification VARCHAR(20) NOT NULL,
        identification_type VARCHAR(20) NOT NULL,
        name VARCHAR(50) NOT NULL,
        last_name VARCHAR(50),
        phonenumber VARCHAR(50) NOT NULL,
        address VARCHAR(50),
        email VARCHAR(50),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        auth_user_id UUID
        REFERENCES auth_users(id)
        ON DELETE CASCADE
    );
