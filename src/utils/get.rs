use gloo_net::http::{Response, Request};
use gloo_net::Error;

#[async_trait(?Send)]
pub trait RequestableItem<P>: Sized {
    fn request(params: P) -> Request;
    async fn response(response: Response) -> Result<Self, Error>;
}

#[async_trait(?Send)]
pub trait Get<P>: Sized {
    async fn get(params: P) -> Self;
}

#[async_trait(?Send)]
impl<I, P> Get<P> for I
where
    I: RequestableItem<P> + 'static,
    P: 'static,
{
    async fn get(params: P) -> Self {
        Self::response(Self::request(params)
            .send()
            .await
            .unwrap()
        )
            .await
            .unwrap()
    }
}