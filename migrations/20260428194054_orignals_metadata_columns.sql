-- Add migration script here
ALTER TABLE originals
ADD COLUMN tags VARCHAR(15)[],
ADD COLUMN release_date TIMESTAMPTZ;