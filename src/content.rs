pub use blog_generic::entities::*;
#[cfg(feature = "client")]
use gloo_net::http::{Request, Response};
#[cfg(feature = "client")]
use gloo_net::Error;
use serde::Deserialize;

use crate::utils::*;

//
// API
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum API<D> {
    Success {
        identifier: String,
        description: Option<String>,
        data: D,
    },
    Failure {
        identifier: String,
        reason: Option<String>,
    },
}

impl<D> ExternalResultContainer for API<D> {
    type Inner = D;
    type Error = String;
    fn result(self) -> Result<Self::Inner, Self::Error> {
        match self {
            API::Success {
                identifier: _,
                description: _,
                data,
            } => Ok(data),
            API::Failure { identifier, reason } => Err(reason.unwrap_or(identifier)),
        }
    }
}

//
// Tokened
//
//

#[derive(Clone, PartialEq)]
pub struct Tokened<P> {
    pub token: String,
    pub params: P,
}

impl<T: Identifiable> Identifiable for Tokened<T> {
    type Id = T::Id;
    fn id(&self) -> &Self::Id {
        if core::any::type_name::<T>() == core::any::type_name::<Tokened<T>>() {
            panic!("recursion")
        }
        self.id()
    }
}

//
// Authors
//
//

#[derive(Clone, PartialEq)]
pub struct AuthorsContainerSearchParams {
    pub query: String,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for API<AuthorsContainer> {
    async fn request(params: ExternalListContainerParams<()>) -> Result<Request, Error> {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let url = format!(
            "{url}/api/authors?limit={limit}&offset={skip}",
            url = crate::API_URL
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<AuthorsContainerSearchParams>>
    for API<AuthorsContainer>
{
    async fn request(
        params: ExternalListContainerParams<AuthorsContainerSearchParams>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            limit,
            skip,
            params: AuthorsContainerSearchParams { query },
        } = params;
        let url = format!(
            "{url}/api/search/authors/{query}?limit={limit}&offset={skip}",
            url = crate::API_URL
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

impl ExternalListContainer for AuthorsContainer {
    type Item = Author;
    fn items(self) -> Vec<Self::Item> {
        self.authors
    }
    fn total(&self) -> u64 {
        self.base.total
    }
    fn skip(&self) -> u64 {
        self.base.offset
    }
    fn limit(&self) -> u64 {
        self.base.limit
    }
}

//
// Author
//
//

#[derive(Clone, PartialEq)]
pub struct AuthorSlugParams {
    pub slug: String,
}

impl Identifiable for AuthorSlugParams {
    type Id = String;
    fn id(&self) -> &Self::Id {
        &self.slug
    }
}

#[derive(Clone, PartialEq)]
pub struct AuthorMeParams;

impl Identifiable for AuthorMeParams {
    type Id = String;
    fn id(&self) -> &Self::Id {
        unreachable!()
    }
}

impl Identifiable for Author {
    type Id = String;
    fn id(&self) -> &Self::Id {
        &self.base.slug
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<AuthorSlugParams> for API<AuthorContainer> {
    async fn request(params: AuthorSlugParams) -> Result<Request, Error> {
        let AuthorSlugParams { slug } = params;
        let url = format!("{url}/api/author/{slug}", url = crate::API_URL);
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<AuthorMeParams>> for API<AuthorContainer> {
    async fn request(params: Tokened<AuthorMeParams>) -> Result<Request, Error> {
        let Tokened { token, params: _ } = params;
        let url = format!("{url}/api/author/me", url = crate::API_URL);
        Ok(Request::get(url.as_str())
            .header("Token", token.as_str())
            .build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

impl ExternalItemContainer for AuthorContainer {
    type Item = Author;
    fn item(self) -> Self::Item {
        self.author
    }
}

//
// Posts
//
//

#[derive(Clone, PartialEq)]
pub struct PostsContainerSearchParam {
    pub query: String,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for API<PostsContainer> {
    async fn request(params: ExternalListContainerParams<()>) -> Result<Request, Error> {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let url = format!(
            "{url}/api/posts?limit={limit}&offset={skip}",
            url = crate::API_URL
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<PostsContainerSearchParam>>
    for API<PostsContainer>
{
    async fn request(
        params: ExternalListContainerParams<PostsContainerSearchParam>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            limit,
            skip,
            params: PostsContainerSearchParam { query },
        } = params;
        let url = format!(
            "{url}/api/search/posts/{query}?limit={limit}&offset={skip}",
            url = crate::API_URL,
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

impl ExternalListContainer for PostsContainer {
    type Item = Post;
    fn items(self) -> Vec<Self::Item> {
        self.posts
    }
    fn total(&self) -> u64 {
        self.base.total
    }
    fn skip(&self) -> u64 {
        self.base.offset
    }
    fn limit(&self) -> u64 {
        self.base.limit
    }
}

impl ExternalResultContainer for PostsContainer {
    type Inner = PostsContainer;
    type Error = std::convert::Infallible;
    fn result(self) -> Result<Self::Inner, Self::Error> {
        Ok(self)
    }
}

//
// Post
//
//

#[derive(Clone, PartialEq)]
pub struct PostIdParams {
    pub id: u64,
}

#[derive(Clone, PartialEq)]
pub struct NewPostParams {
    pub new_post: CommonPost,
}

#[derive(Clone, PartialEq)]
pub struct UpdatePostParams {
    pub id: u64,
    pub update_post: CommonPost,
}

impl Identifiable for PostIdParams {
    type Id = u64;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Identifiable for Post {
    type Id = u64;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<PostIdParams> for API<PostContainer> {
    async fn request(params: PostIdParams) -> Result<Request, Error> {
        let PostIdParams { id } = params;
        let url = format!("{url}/api/post/{id}", url = crate::API_URL);
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<NewPostParams>> for API<PostContainer> {
    async fn request(params: Tokened<NewPostParams>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: NewPostParams { new_post },
        } = params;
        let url = format!("{url}/api/post", url = crate::API_URL);
        Ok(Request::post(url.as_str())
            .header("Token", token.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&new_post).map_err(|e| Error::SerdeError(e))?)?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<UpdatePostParams>> for API<PostContainer> {
    async fn request(params: Tokened<UpdatePostParams>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: UpdatePostParams { id, update_post },
        } = params;
        let url = format!("{url}/api/post/{id}", url = crate::API_URL);
        Ok(Request::patch(url.as_str())
            .header("Token", token.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&update_post).map_err(|e| Error::SerdeError(e))?)?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

impl ExternalItemContainer for PostContainer {
    type Item = Post;
    fn item(self) -> Self::Item {
        self.post
    }
}

//
// Comments
//
//

#[derive(Clone, PartialEq)]
pub struct CommentsContainerPostIdParams {
    pub post_id: u64,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<CommentsContainerPostIdParams>>
    for API<CommentsContainer>
{
    async fn request(
        params: ExternalListContainerParams<CommentsContainerPostIdParams>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            params: CommentsContainerPostIdParams { post_id },
            limit,
            skip,
        } = params;
        let url = format!(
            "{url}/api/comments/{post_id}?limit={limit}&offset={skip}",
            url = crate::API_URL,
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

impl ExternalListContainer for CommentsContainer {
    type Item = Comment;
    fn items(self) -> Vec<Self::Item> {
        self.comments
    }
    fn total(&self) -> u64 {
        self.base.total
    }
    fn skip(&self) -> u64 {
        self.base.offset
    }
    fn limit(&self) -> u64 {
        self.base.limit
    }
}

impl ExternalResultContainer for CommentsContainer {
    type Inner = CommentsContainer;
    type Error = std::convert::Infallible;
    fn result(self) -> Result<Self::Inner, Self::Error> {
        Ok(self)
    }
}

//
// Login
//
//

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<LoginQuestion> for API<LoginAnswer> {
    async fn request(params: LoginQuestion) -> Result<Request, Error> {
        let url = format!("{url}/api/login", url = crate::API_URL);
        Ok(Request::post(url.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).map_err(|e| Error::SerdeError(e))?)?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}
