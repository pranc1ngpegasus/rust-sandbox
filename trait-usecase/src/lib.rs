use std::future::Future;
use std::pin::Pin;

pub trait Usecase<Input: ?Sized, Output>: Send + Sync + 'static {
    fn handle<'a>(
        &'a self,
        input: &'a Input,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Output>> + Send + 'a>>;
}
