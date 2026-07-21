-- Add migration script here
CREATE TABLE wall_post_reactions(
  profile_id UUID NOT NULL,
  wall_post_id UUID NOT NULL,
  reaction VARCHAR(10) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (profile_id, wall_post_id),
  CONSTRAINT fk_reaction_profile FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
  CONSTRAINT fk_reaction_wall_post FOREIGN KEY (wall_post_id) REFERENCES wall_posts(id) ON DELETE CASCADE
);
CREATE TABLE discussion_post(
  id UUID PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  author_id UUID,
  work_id UUID,
  set_id UUID,
  total_reactions BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_active TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_author FOREIGN KEY (author_id) REFERENCES profiles(id) ON DELETE SET NULL,
  CONSTRAINT fk_work FOREIGN KEY (work_id) REFERENCES works(id) ON DELETE SET NULL,
  CONSTRAINT fk_set FOREIGN KEY (set_id) REFERENCES sets(id) ON DELETE SET NULL
);
CREATE TABLE discussion_comments(
  id UUID PRIMARY KEY,
  discussion_post_id UUID NOT NULL,
  parent_id UUID,
  content TEXT NOT NULL,
  author_id UUID,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  is_deleted bool NOT NULL DEFAULT false,
  CONSTRAINT fk_discussion_post FOREIGN KEY (discussion_post_id) REFERENCES discussion_post(id) ON DELETE CASCADE,
  CONSTRAINT fk_parent_comment FOREIGN KEY (parent_id) REFERENCES discussion_comments(id) ON DELETE CASCADE,
  CONSTRAINT fk_comment_author FOREIGN KEY (author_id) REFERENCES profiles(id) ON DELETE SET NULL
);
ALTER TABLE spirit
ADD COLUMN token_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE recommendation_stars
  RENAME TO recommendation_boosts;