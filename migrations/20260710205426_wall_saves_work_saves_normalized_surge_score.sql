-- Add migration script here
ALTER TABLE library
ADD COLUMN normalized_score INTEGER;
ALTER TABLE wall_posts
ADD COLUMN recommendation_id UUID REFERENCES recommendations(id);
ALTER TABLE wall_posts
ADD COLUMN original_id UUID REFERENCES originals(id);
ALTER TABLE wall_posts
ADD COLUMN total_views BIGINT NOT NULL DEFAULT 0;
ALTER TABLE wall_posts
ADD COLUMN total_saves BIGINT NOT NULL DEFAULT 0;
ALTER TABLE wall_posts
ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE wall_posts
ALTER COLUMN created_at
SET NOT NULL;
ALTER TABLE wall_posts DROP CONSTRAINT chk_text_line;
ALTER TABLE wall_posts
ADD CONSTRAINT chk_wall_post_content CHECK (
    text_line IS NOT NULL
    OR work_id IS NOT NULL
    OR recommendation_id IS NOT NULL
    OR original_id IS NOT NULL
  );
ALTER TABLE wall_posts
ADD CONSTRAINT chk_wall_post_single_reference CHECK (
    num_nonnulls(work_id, recommendation_id, original_id) <= 1
  );
CREATE TABLE wall_post_saves (
  wall_post_id UUID REFERENCES wall_posts(id) ON DELETE CASCADE,
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, wall_post_id)
);
CREATE TABLE recommendation_stars (
  recommendation_id UUID REFERENCES recommendations(id) ON DELETE CASCADE,
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, recommendation_id)
);
CREATE TABLE spirit (
  fan UUID REFERENCES profiles(id) ON DELETE CASCADE,
  artist UUID REFERENCES profiles(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (fan, artist)
);
DROP TABLE work_quotes;