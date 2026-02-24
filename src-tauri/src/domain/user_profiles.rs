const DEV_USER_NAME: &str = "dev";
const DEV_USER_EMAIL: &str = "dev@example.com";

crate::domain::define_domain! {
    UserProfiles,
    "user_profiles",
    user_profiles,
    {
        user_name: String = DEV_USER_NAME.to_string(),
        email: String = DEV_USER_EMAIL.to_string(),
    }
}
