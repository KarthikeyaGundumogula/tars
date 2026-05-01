-- Add migration script here
CREATE TYPE original_category AS ENUM ('MOVIE', 'SERIES');
CREATE TYPE ledger_entry_type AS ENUM ('MOVIE', 'SERIES', 'EPISODE', 'SEASON');
ALTER TABLE originals
ADD COLUMN genres VARCHAR(15) [],
  ADD COLUMN release_date TIMESTAMPTZ,
  ADD COLUMN duration VARCHAR(20),
  ADD COLUMN parent UUID;
CREATE TABLE episodes (
  id UUID NOT NULL PRIMARY KEY,
  season_id UUID NOT NULL,
  original_id UUID NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  ep_number INT DEFAULT 1,
  ep_release_date TIMESTAMPTZ,
  created_at TIMESTAMPTZ,
  CONSTRAINT fk_season FOREIGN KEY (season_id) REFERENCES originals (id) ON DELETE CASCADE,
  CONSTRAINT fk_orignal FOREIGN KEY (original_id) REFERENCES originals (id) ON DELETE CASCADE
);
ALTER TABLE ledger DROP CONSTRAINT ledger_pkey CASCADE;
ALTER TABLE ledger ALTER COLUMN original_id DROP NOT NULL; 
ALTER TABLE ledger
ADD COLUMN entry_type ledger_entry_type NOT NULL,
  ADD COLUMN episode_id UUID,
  ADD COLUMN id UUID PRIMARY KEY,
  ADD CONSTRAINT one_watchable CHECK (
    (
      original_id IS NULL
      AND episode_id IS NOT NULL
    )
    OR (
      original_id IS NOT NULL
      AND episode_id IS NULL
    )
  );