-- Add up migration script here


CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS products (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(20) NOT NULL,
        description VARCHAR(20) NOT NULL,
        price INT NOT NULL,
        stock INT NOT NULL,
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
