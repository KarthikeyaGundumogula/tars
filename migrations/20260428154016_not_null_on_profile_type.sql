-- Add migration script here
ALTER TABLE profiles
ALTER COLUMN profile_type
SET NOT NULL;