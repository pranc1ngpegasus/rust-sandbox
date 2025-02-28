use std::future::Future;
use std::pin::Pin;

pub trait Usecase: Send + Sync + 'static {
    type Input: ?Sized;
    type Output;

    fn handle<'a>(
        &'a self,
        input: &'a Self::Input,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Self::Output>> + Send + 'a>>;
}
