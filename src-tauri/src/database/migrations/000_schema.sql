CREATE TABLE IF NOT EXISTS user_profile (
    user_id TEXT PRIMARY KEY,
    user_name TEXT NOT NULL,
    email TEXT NOT NULL,
)
CREATE TABLE IF NOT EXISTS user_preferences (
    user_id TEXT PRIMARY KEY,
    is_sidebar_open BOOLEAN NOT NULL DEFAULT TRUE
)