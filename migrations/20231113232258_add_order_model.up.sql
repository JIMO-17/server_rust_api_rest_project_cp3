-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS orders (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        order_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        quantity INT NOT NULL,
        total INT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        admin_id UUID
        REFERENCES admins(id)
        ON DELETE CASCADE,
        customer_id UUID
        REFERENCES customers(id)
        ON DELETE CASCADE
    );