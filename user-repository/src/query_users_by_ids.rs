use derive_new::new;
use sqlx::query_file_as;
use trait_repository::Repository;
use user_model::user::User;
use user_model::user_id::UserID;

#[derive(new)]
pub struct QueryUsersByIDs {}

impl Repository<[UserID], Vec<User>> for QueryUsersByIDs {
    fn handle<'a>(
        &'a self,
        conn: &'a mut sqlx::PgConnection,
        input: &'a [UserID],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Vec<User>>> + Send + 'a>>
    {
        Box::pin(async move {
            let ids: Vec<uuid::Uuid> = input.iter().map(|user| user.inner().to_owned()).collect();

            query_file_as!(User, "queries/query_users_by_ids.sql", &ids)
                .fetch_all(&mut *conn)
                .await
                .map_err(|e| anyhow::anyhow!("failed to fetch user by id: {}", e))
        })
    }
}
