-- Add migration script here
ALTER TABLE profiles
ADD COLUMN stage_name TEXT,
  ADD COLUMN text_color TEXT,
  ADD COLUMN background_color TEXT;
ALTER TABLE edits DROP COLUMN created_at;
ALTER TABLE posters DROP COLUMN created_at;
ALTER TABLE scripts DROP COLUMN created_at;
CREATE TABLE favorite_profiles(
  profile_id UUID NOT NULL,
  favorited_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id),
  CONSTRAINT fk_favorited_id FOREIGN KEY (favorited_id) REFERENCES profiles(id),
  PRIMARY KEY (profile_id, favorited_id)
);
CREATE TABLE followings(
  follower_id UUID NOT NULL,
  following_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_follower_id FOREIGN KEY (follower_id) REFERENCES profiles(id),
  CONSTRAINT fk_following_id FOREIGN KEY (following_id) REFERENCES profiles(id),
  PRIMARY KEY (follower_id, following_id)
);
CREATE TABLE sets(
  id UUID NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  statement TEXT NOT NULL,
  description TEXT NOT NULL,
  presence BIGINT NOT NULL,
  curator UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_curator FOREIGN KEY (curator) REFERENCES profiles(id)
);
CREATE TYPE set_role AS ENUM ('CURATOR', 'MEMBER');
CREATE TABLE set_members(
  profile_id UUID NOT NULL,
  set_id UUID NOT NULL,
  set_role set_role NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id),
  CONSTRAINT fk_set_id FOREIGN KEY (set_id) REFERENCES sets(id),
  PRIMARY KEY (profile_id, set_id)
);
CREATE TABLE festivals(
  id UUID NOT NULL PRIMARY KEY,
  set_id UUID NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  start_date TIMESTAMPTZ NOT NULL,
  end_date TIMESTAMPTZ NOT NULL,
  organizer UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_organizer_membership FOREIGN KEY (organizer,set_id) REFERENCES set_members(profile_id,set_id),
  CONSTRAINT check_dates CHECK (start_date<end_date)
);
CREATE TABLE panelists(
  festival_id UUID NOT NULL,
  profile_id UUID NOT NULL,
  work_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_festival_id FOREIGN KEY (festival_id) REFERENCES festivals(id),
  CONSTRAINT fk_profile_id FOREIGN KEY (profile_id) REFERENCES profiles(id),
  CONSTRAINT fk_work_id FOREIGN KEY (work_id) REFERENCES works(id),
  PRIMARY KEY (festival_id, profile_id)
);
CREATE TABLE festival_works(
  festival_id UUID NOT NULL,
  work_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_festival_id FOREIGN KEY (festival_id) REFERENCES festivals(id),
  CONSTRAINT fk_work_id FOREIGN KEY (work_id) REFERENCES works(id),
  PRIMARY KEY (festival_id, work_id)
);
CREATE TABLE set_works(
  set_id UUID NOT NULL,
  work_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_set_id FOREIGN KEY (set_id) REFERENCES sets(id),
  CONSTRAINT fk_work_id FOREIGN KEY (work_id) REFERENCES works(id),
  PRIMARY KEY (set_id, work_id)
);