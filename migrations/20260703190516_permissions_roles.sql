-- Add migration script here
CREATE TABLE user_roles(
  name VARCHAR(255) PRIMARY KEY,
  description TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE permissions (
  name VARCHAR(255) PRIMARY KEY,
  description TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE role_permissions(
  role_name VARCHAR(255) REFERENCES user_roles(name) ON DELETE CASCADE,
  permission_name VARCHAR(255) REFERENCES permissions(name) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (role_name, permission_name)
);
ALTER TABLE profiles
ADD COLUMN role_name VARCHAR(255) REFERENCES user_roles(name) ON DELETE
SET NULL;
DROP TABLE beta_whitelist;
ALTER TABLE profiles
ADD COLUMN color_theme VARCHAR(20) NOT NULL DEFAULT '#FFFFFF';
ALTER TABLE profiles DROP COLUMN text_color;
ALTER TABLE profiles DROP COLUMN background_color;
ALTER TABLE profiles
  RENAME COLUMN presence TO spirit;