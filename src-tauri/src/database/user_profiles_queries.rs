use crate::{database::impl_crud, domain::user_profiles::UserProfiles};

impl_crud!(
    UserProfiles,
    "user_profiles",
    user_profiles,
    {user_name, email}
);
