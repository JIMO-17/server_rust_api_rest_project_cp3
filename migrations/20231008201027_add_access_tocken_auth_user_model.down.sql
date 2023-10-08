-- Add down migration script here

ALTER TABLE 
    auth_users
DROP
    COLUMN access_token;
