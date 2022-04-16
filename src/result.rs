use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub trait FutureResult<T, E> where T: Send + 'static, E: Send + 'static {
    async fn async_and_then<OutT, F, Fut>(self, op: F) -> Result<OutT, E>
    where F: FnOnce(T) -> Fut + Send,
        Fut: Future<Output = Result<OutT, E>> + Send + 'static;
}

#[async_trait]
impl<T, E> FutureResult<T, E> for Result<T, E> where T: Send + 'static, E: Send + 'static {
    async fn async_and_then<OutT, F, Fut>(self, op: F) -> Result<OutT, E>
    where F: FnOnce(T) -> Fut + Send,
        Fut: Future<Output = Result<OutT, E>> + Send + 'static {
        match self {
            Ok(fut) => op(fut).await,
            Err(err) => Err(err),
        }
    }
}
