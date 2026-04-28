-- Add migration script here
CREATE TYPE profile_type AS ENUM('STAR', 'MAKER', 'ARTIST');
ALTER TABLE originals DROP COLUMN releases;
ALTER TABLE originals
ADD COLUMN associated_with UUID;
ALTER TABLE originals
ADD CONSTRAINT associated_with_fk FOREIGN KEY (associated_with) REFERENCES profiles (id) ON DELETE
SET NULL;
ALTER TABLE profiles
ALTER COLUMN is_claimed
SET NOT NULL;
ALTER TABLE profiles
ADD COLUMN presence BIGINT NOT NULL DEFAULT 100,
  ADD COLUMN profile_type profile_type DEFAULT 'ARTIST';
DELETE FROM profiles
WHERE created_at NOT IN (
    SELECT MIN(created_at)
    FROM profiles
    GROUP BY user_name
  );
ALTER TABLE profiles
ADD CONSTRAINT profiles_user_name_unique UNIQUE (user_name);