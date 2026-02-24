CREATE TABLE IF NOT EXISTS user_profiles (
    user_id TEXT PRIMARY KEY,
    user_name TEXT NOT NULL,
    email TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_preferences (
    user_id TEXT PRIMARY KEY,
    is_sidebar_open BOOLEAN NOT NULL DEFAULT TRUE,
    enable_complete_input BOOLEAN NOT NULL DEFAULT TRUE,
    enable_proofread BOOLEAN NOT NULL DEFAULT TRUE
);