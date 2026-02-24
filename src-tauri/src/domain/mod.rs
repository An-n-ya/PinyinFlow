pub(crate) mod preferences;
pub(crate) mod user_profiles;

macro_rules! define_domain {
    (
        $struct_name:ident,
        $table_name:expr,
        $suffix:ident,
        { $($name:ident : $type:ty = $default:expr),* $(,)? }
    ) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, sqlx::FromRow)]
        #[serde(rename_all = "camelCase")]
        pub struct $struct_name {
            pub user_id: String,
            $(pub $name: $type,)*
        }

        impl $struct_name {
            pub fn dev(user_id: &str) -> Self {
                Self {
                    user_id: user_id.to_string(),
                    $($name: $default,)*
                }
            }
        }

        crate::database::impl_crud!(
            $struct_name,
            $table_name,
            $suffix,
            { $($name),* }
        );

    };
}

pub(crate) use define_domain;
