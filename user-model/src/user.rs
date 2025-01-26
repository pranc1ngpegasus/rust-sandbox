use crate::user_id::UserID;
use derive_new::new;

#[derive(Clone, Debug, PartialEq, new, sqlx::FromRow)]
pub struct User {
    #[new(value = "UserID::new()")]
    pub id: UserID,
}
