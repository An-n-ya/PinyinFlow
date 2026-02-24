use serde::{Deserialize, Serialize};

macro_rules! define_preferences {
    ($($name:ident : $type:ty = $default:expr),* $(,)?) => {
        #[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
        #[serde(rename_all = "camelCase")]
        pub struct UserPreferences {
            pub user_id: String,
            $(pub $name: $type,)*
        }

        impl UserPreferences {
            pub fn dev(user_id: &str) -> Self {
                Self {
                    user_id: user_id.to_string(),
                    $($name: $default,)*
                }
            }
        }
    };
}

define_preferences! {
    is_sidebar_open: bool = true,
    enable_complete_input: bool = true,
    enable_proofread: bool = true,
}
