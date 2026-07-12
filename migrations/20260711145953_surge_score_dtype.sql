-- Add migration script here
ALTER TABLE library DROP COLUMN normalized_score;
ALTER TABLE library
ADD COLUMN peak_snapshot bigint NOT NULL;