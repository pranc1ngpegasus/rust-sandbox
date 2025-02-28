use derive_new::new;
use sqlx::{Pool, Postgres};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use trait_repository::Repository;
use trait_usecase::Usecase;
use user_model::user::User;
use user_model::user_id::UserID;

#[derive(new)]
pub struct QueryUsersByIDs<R>
where
    R: Repository<Input = [UserID], Output = Vec<User>>,
{
    pub reader_pool: Pool<Postgres>,
    pub repository: Arc<R>,
}

#[derive(Debug)]
pub struct Input {
    pub ids: Vec<UserID>,
}

#[derive(Debug)]
pub struct Output {
    pub users: Vec<User>,
}

impl<R> Usecase for QueryUsersByIDs<R>
where
    R: Repository<Input = [UserID], Output = Vec<User>>,
{
    type Input = Input;
    type Output = Output;

    fn handle<'a>(
        &'a self,
        input: &'a Self::Input,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Self::Output>> + Send + 'a>> {
        Box::pin(async move {
            let mut conn = self.reader_pool.acquire().await?;

            let users = self.repository.handle(&mut conn, &input.ids).await?;

            Ok(Output { users })
        })
    }
}
