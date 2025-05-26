-- This file should undo anything in `up.sql`
-- down.sql (for re-adding role column)

ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'user'; -- Adjust type and default if different