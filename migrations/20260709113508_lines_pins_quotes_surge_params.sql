-- Add migration script here

CREATE TABLE wall_posts(
  id UUID PRIMARY KEY,
  artist_id UUID NOT NULL,
  text_line TEXT,
  work_id UUID,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  CONSTRAINT fk_artist FOREIGN KEY (artist_id) REFERENCES profiles(id) ON DELETE CASCADE,
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE SET NULL,
  CONSTRAINT chk_text_line CHECK (text_line IS NOT NULL OR work_id IS NOT NULL)
);

CREATE TABLE work_pins(
  work_id UUID NOT NULL,
  wall_post_id UUID NOT NULL,
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE,
  CONSTRAINT fk_pinned_wall_post FOREIGN KEY (wall_post_id) REFERENCES wall_posts(id) ON DELETE CASCADE
);

CREATE TABLE work_quotes(
  work_id UUID NOT NULL,
  wall_post_id UUID NOT NULL,
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE CASCADE,
  CONSTRAINT fk_quoted_wall_post FOREIGN KEY (wall_post_id) REFERENCES wall_posts(id) ON DELETE CASCADE
);

  ALTER TABLE profiles ALTER COLUMN role_name SET NOT NULL;

ALTER TABLE originals ADD COLUMN number_of_surges INTEGER DEFAULT 0;
ALTER TABLE originals ADD COLUMN surge_m2 FLOAT DEFAULT 0.0;
ALTER TABLE originals ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE originals RENAME COLUMN resonance_density TO mean_surge;

ALTER TABLE work_likes RENAME TO work_stars;