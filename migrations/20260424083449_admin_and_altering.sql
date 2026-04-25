-- Add migration script here
CREATE TYPE watchlist_status AS ENUM ('WATCHED', 'WATCHING', 'WANT_TO_WATCH');
CREATE TABLE ADMINS (
  admin_id UUID PRIMARY KEY,
  admin_name TEXT NOT NULL,
  admin_password_hash TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);
CREATE TABLE BETA_WHITELIST (
  artist_username TEXT NOT NULL,
  is_claimed BOOLEAN NOT NULL,
  added_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);
ALTER TABLE PROFILES
ADD COLUMN profile_picture TEXT NOT NULL,
  ADD COLUMN password_hash TEXT NOT NULL;
ALTER TABLE SCRIPTS
ALTER COLUMN thoughts TYPE TEXT [] USING ARRAY[thoughts];
ALTER TABLE originals
ADD COLUMN password_hash TEXT NOT NULL;
CREATE TABLE ledger (
  original_id UUID NOT NULL,
  profile_id UUID NOT NULL,
  pub_visibility BOOLEAN DEFAULT true,
  tagged_works UUID [],
  pre_thought TEXT,
  post_impression TEXT,
  status watchlist_status DEFAULT 'WANT_TO_WATCH',
  CONSTRAINT fk_original FOREIGN KEY (original_id) REFERENCES originals (id) ON DELETE CASCADE,
  CONSTRAINT fk_profile FOREIGN KEY (profile_id) REFERENCES profiles (id) ON DELETE CASCADE,
  PRIMARY KEY (profile_id, original_id)
);