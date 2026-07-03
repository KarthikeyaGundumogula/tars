-- Add migration script here
-- Recommendations
CREATE TABLE recommendations(
  id UUID PRIMARY KEY,
  original_id UUID NOT NULL,
  artist_id UUID NOT NULL,
  notes TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  surge_score BIGINT NOT NULL DEFAULT 1,
  boost_number BIGINT NOT NULL DEFAULT 0,
  saves BIGINT NOT NULL DEFAULT 0,
  CONSTRAINT fk_original FOREIGN KEY (original_id) REFERENCES originals(id) ON DELETE CASCADE,
  CONSTRAINT fk_artist FOREIGN KEY (artist_id) REFERENCES profiles(id) ON DELETE CASCADE,
  CONSTRAINT check_surge_score_positive CHECK (surge_score >= 0),
  CONSTRAINT check_boost_number_positive CHECK (boost_number >= 0)
);
CREATE TABLE saved_recommendations(
  recommendation_id UUID NOT NULL,
  artist_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_recommendation FOREIGN KEY (recommendation_id) REFERENCES recommendations(id) ON DELETE CASCADE,
  CONSTRAINT fk_artist FOREIGN KEY (artist_id) REFERENCES profiles(id) ON DELETE CASCADE,
  PRIMARY KEY (recommendation_id, artist_id)
);
CREATE TABLE saved_works(
  work_id UUID NOT NULL,
  artist_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE,
  CONSTRAINT fk_artist FOREIGN KEY (artist_id) REFERENCES profiles(id) ON DELETE CASCADE,
  PRIMARY KEY (work_id, artist_id)
);
ALTER TABLE ledger
ADD COLUMN surge_score BIGINT NOT NULL DEFAULT 0;
ALTER TABLE ledger
  RENAME TO library;
ALTER TYPE ledger_entry_type
RENAME TO library_entry_type;
ALTER TABLE originals
ADD COLUMN resonance_density BIGINT NOT NULL DEFAULT 0;
ALTER TABLE originals
ADD COLUMN surge_spread BIGINT NOT NULL DEFAULT 0;
ALTER TABLE originals
ADD COLUMN film_certification TEXT NOT NULL DEFAULT '';
ALTER TABLE sets DROP COLUMN profile_picture;
ALTER TABLE sets
ADD COLUMN color_theme TEXT NOT NULL DEFAULT '';
ALTER TABLE profiles
ADD COLUMN current_peak_recommendations BIGINT NOT NULL DEFAULT 0;
ALTER TABLE profiles
ADD COLUMN current_peak_library BIGINT NOT NULL DEFAULT 0;
ALTER TABLE originals_credits
  RENAME TO work_credits;
ALTER TABLE works
  RENAME COLUMN credits TO stars;
ALTER TABLE works
ADD COLUMN saves BIGINT NOT NULL DEFAULT 0;
DROP TABLE followings;