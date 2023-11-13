-- Add up migration script here

-- delete column auth_user_id from products
ALTER TABLE products DROP COLUMN auth_user_id;