use sqlx::PgConnection;
use std::future::Future;
use std::pin::Pin;

pub trait Repository<Input: ?Sized, Output>: Send + Sync + 'static {
    fn handle<'a>(
        &'a self,
        conn: &'a mut PgConnection,
        input: &'a Input,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Output>> + Send + 'a>>;
}
