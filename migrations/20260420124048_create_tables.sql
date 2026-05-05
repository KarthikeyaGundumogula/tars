-- Add migration script here
CREATE TYPE role_type AS ENUM ('STAR', 'MAKER');
CREATE TYPE supported_platforms AS ENUM ('YOUTUBE', 'TWITTER', 'NATIVE');
CREATE TYPE edit_format AS ENUM (
  'IMAX-VIDEO',
  'ACADEMY-VIDEO',
  'SQUARE-VIDEO',
  'VERTICAL-VIDEO'
);
CREATE TYPE poster_format as ENUM (
  'CANVAS-POSTER',
  'STANDARD-POSTER',
  'SQUARE-POSTER',
  'VERTICAL-POSTER'
);
CREATE TYPE work_category AS ENUM ('EDIT', 'POSTER', 'SCRIPT');
CREATE TABLE originals (
  id UUID NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  cover_img VARCHAR NOT NULL,
  presence BIGINT DEFAULT 100,
  releases BIGINT DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);
CREATE TABLE profiles (
  id UUID NOT NULL PRIMARY KEY,
  user_name VARCHAR(50) NOT NULL,
  tag_line TEXT NOT NULL,
  is_claimed bool DEFAULT false,
  youtube_profile TEXT,
  twitter_profile TEXT,
  instagram_profile TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);
CREATE TABLE roles (
  profile_id UUID NOT NULL,
  original_id UUID NOT NULL,
  category role_type NOT NULL,
  role_name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
  CONSTRAINT fk_profile FOREIGN KEY (profile_id) REFERENCES profiles (id) ON DELETE CASCADE,
  CONSTRAINT fk_original FOREIGN KEY (original_id) REFERENCES originals (id) ON DELETE CASCADE,
  PRIMARY KEY (profile_id, original_id, role_name)
);
CREATE TABLE works (
  id UUID PRIMARY KEY,
  title VARCHAR,
  artist_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  credits BIGINT DEFAULT 0,
  category work_category NOT NULL,
  CONSTRAINT fk_artist FOREIGN KEY (artist_id) REFERENCES profiles(id) ON DELETE CASCADE
);
CREATE TABLE originals_credits(
  work_id UUID NOT NULL,
  original_id UUID NOT NULL,
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works (id) ON DELETE CASCADE,
  CONSTRAINT fk_original FOREIGN KEY (original_id) REFERENCES originals (id) ON DELETE CASCADE,
  PRIMARY KEY(work_id, original_id)
);
CREATE TABLE EDITS (
  work_id UUID NOT NULL PRIMARY KEY,
  src_id VARCHAR NOT NULL,
  platform supported_platforms NOT NULL,
  format edit_format NOT NULL DEFAULT 'ACADEMY-VIDEO',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE
);
CREATE TABLE POSTERS (
  work_id UUID PRIMARY KEY,
  src_id VARCHAR NOT NULL,
  format poster_format NOT NULL DEFAULT 'STANDARD-POSTER',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE
);
CREATE TABLE SCRIPTS (
  work_id UUID PRIMARY KEY,
  img_src_ids VARCHAR [],
  CONSTRAINT max_images CHECK (array_length(img_src_ids, 1) <= 10),
  thoughts text,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE
);