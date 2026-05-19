-- Add migration script here

ALTER TABLE ledger DROP CONSTRAINT fk_orignal;
ALTER TABLE ledger
    ADD CONSTRAINT fk_original FOREIGN KEY (original_id) REFERENCES originals(id) ON DELETE CASCADE;

-- ---------------------------------------------------------------------------
-- NOT NULL enforcements
-- ---------------------------------------------------------------------------
ALTER TABLE ledger ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE ledger ALTER COLUMN updated_at SET NOT NULL;
ALTER TABLE ledger ALTER COLUMN status SET NOT NULL;

UPDATE originals SET presence = 100 WHERE presence IS NULL;
ALTER TABLE originals ALTER COLUMN presence SET NOT NULL;

UPDATE sets SET profile_picture = '' WHERE profile_picture IS NULL;
ALTER TABLE sets ALTER COLUMN profile_picture SET NOT NULL;

UPDATE works SET credits = 0 WHERE credits IS NULL;
ALTER TABLE works ALTER COLUMN credits SET NOT NULL;

UPDATE profiles SET stage_name = 'Unknown' WHERE stage_name IS NULL;
UPDATE profiles SET text_color = '#000000' WHERE text_color IS NULL;
UPDATE profiles SET background_color = '#FFFFFF' WHERE background_color IS NULL;
ALTER TABLE profiles ALTER COLUMN stage_name SET NOT NULL;
ALTER TABLE profiles ALTER COLUMN text_color SET NOT NULL;
ALTER TABLE profiles ALTER COLUMN background_color SET NOT NULL;

ALTER TABLE roles RENAME TO cast_and_crew_roles;

ALTER TABLE admins ALTER COLUMN admin_name SET NOT NULL;