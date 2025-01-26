use derive_new::new;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use trait_repository::Repository;
use trait_usecase::Usecase;
use user_model::user::User;
use user_model::user_id::UserID;

#[derive(new)]
pub struct QueryUsersByIDs {
    pub reader_pool: Pool<Postgres>,
    pub repository: Arc<dyn Repository<[UserID], Vec<User>>>,
}

#[derive(Debug)]
pub struct Input {
    pub ids: Vec<UserID>,
}

#[derive(Debug)]
pub struct Output {
    pub users: Vec<User>,
}

impl Usecase<Input, Output> for QueryUsersByIDs {
    fn handle<'a>(
        &'a self,
        input: &'a Input,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Output>> + Send + 'a>>
    {
        Box::pin(async move {
            let mut conn = self.reader_pool.acquire().await?;

            let users = self.repository.handle(&mut conn, &input.ids).await?;

            Ok(Output { users })
        })
    }
}
