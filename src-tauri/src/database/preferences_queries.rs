use crate::{database::impl_crud, domain::preferences::UserPreferences};

impl_crud!(
    UserPreferences,
    "user_preferences",
    user_preferences,
    {is_sidebar_open, enable_complete_input, enable_proofread}
);
