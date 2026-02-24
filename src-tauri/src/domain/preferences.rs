crate::domain::define_domain! {
    UserPreferences,
    "user_preferences",
    user_preferences,
    {
        is_sidebar_open: bool = true,
        enable_complete_input: bool = true,
        enable_proofread: bool = true,
    }
}
