use gloo_net::http::{Request, Response};
use gloo_net::Error;
use serde::{Deserialize, Serialize};

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
// AuthorsContainer
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorsContainer {
    pub authors: Vec<Author>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for API<AuthorsContainer> {
    async fn request(params: ExternalListContainerParams<()>) -> Result<Request, Error> {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let url = format!("http://127.0.0.1:3000/api/authors?limit={limit}&offset={skip}");
        Ok(Request::get(url.as_str()))
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
        self.total
    }
    fn skip(&self) -> u64 {
        self.offset
    }
    fn limit(&self) -> u64 {
        self.limit
    }
}

//
// Author
//
//

#[derive(Clone, PartialEq)]
pub struct AuthorSlugParam {
    pub slug: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub slug: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub registered_at: i64,
    pub status: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorContainer {
    pub author: Author,
}

#[async_trait(?Send)]
impl RequestableItem<AuthorSlugParam> for API<AuthorContainer> {
    async fn request(params: AuthorSlugParam) -> Result<Request, Error> {
        let AuthorSlugParam { slug } = params;
        let url = format!("http://127.0.0.1:3000/api/author/{slug}");
        Ok(Request::get(url.as_str()))
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
// PostsContainer
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct PostsContainer {
    pub posts: Vec<Post>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for API<PostsContainer> {
    async fn request(params: ExternalListContainerParams<()>) -> Result<Request, Error> {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let url = format!("http://127.0.0.1:3000/api/posts?limit={limit}&skip={skip}");
        Ok(Request::get(url.as_str()))
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
        self.total
    }
    fn skip(&self) -> u64 {
        self.offset
    }
    fn limit(&self) -> u64 {
        self.limit
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
pub struct PostSlugParam {
    pub slug: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub title: String,
    pub slug: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortAuthor {
    pub slug: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub created_at: i64,
    pub content: Option<String>,
    pub short_author: ShortAuthor,
    pub tags: Vec<Tag>,
}

impl Post {
    pub fn image_url(&self) -> String {
        format!(
            "https://source.unsplash.com/random/{}x{}?{}&sig={}",
            400,
            100,
            self.tags.clone().into_iter().map(|v| v.title).collect::<Vec<String>>().join(","),
            self.slug,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContainer {
    pub post: Post,
}

#[async_trait(?Send)]
impl RequestableItem<PostSlugParam> for API<PostContainer> {
    async fn request(params: PostSlugParam) -> Result<Request, Error> {
        let PostSlugParam { slug } = params;
        let url = format!("http://127.0.0.1:3000/api/post/{slug}");
        Ok(Request::get(url.as_str()))
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
// CommentsContainer
//
//

#[derive(Clone, PartialEq)]
pub struct CommentsContainerPostIdParam {
    pub post_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct CommentsContainer {
    pub comments: Vec<Comment>,
    pub total: u64,
    pub skip: u64,
    pub limit: u64,
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<CommentsContainerPostIdParam>>
    for CommentsContainer
{
    async fn request(
        params: ExternalListContainerParams<CommentsContainerPostIdParam>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            params,
            limit,
            skip,
        } = params;
        let url = format!(
            "https://dummyjson.com/comments/post/{post_id}?limit={limit}&skip={skip}",
            post_id = params.post_id,
        );
        Ok(Request::get(url.as_str()))
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for CommentsContainer {
    async fn request(params: ExternalListContainerParams<()>) -> Result<Request, Error> {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let url = format!("https://dummyjson.com/comments?limit={limit}&skip={skip}");
        Ok(Request::get(url.as_str()))
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
        self.total
    }
    fn skip(&self) -> u64 {
        self.skip
    }
    fn limit(&self) -> u64 {
        self.limit
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
// Comment
//
//

#[derive(Clone, PartialEq)]
pub struct CommentsPostIdParam {
    pub post_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct ShortUser {
    pub id: u64,
    pub username: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub body: String,
    #[serde(rename = "postId")]
    pub post_id: u64,
    #[serde(rename = "user")]
    pub short_user: ShortUser,
}

#[async_trait(?Send)]
impl RequestableItem<CommentsPostIdParam> for Comment {
    async fn request(params: CommentsPostIdParam) -> Result<Request, Error> {
        let url = format!("https://dummyjson.com/comments/{id}", id = params.post_id);
        Ok(Request::get(url.as_str()))
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

impl ExternalItemContainer for Comment {
    type Item = Self;
    fn item(self) -> Self::Item {
        self
    }
}

impl ExternalResultContainer for Comment {
    type Inner = Comment;
    type Error = std::convert::Infallible;
    fn result(self) -> Result<Self::Inner, Self::Error> {
        Ok(self)
    }
}

//
// Login
//
//

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AuthUser {
    pub id: u64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "image")]
    pub image_url: String,
    pub username: String,
    pub email: String,
    pub token: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum AuthResult {
    Success(AuthUser),
    Error { message: String },
}

#[async_trait(?Send)]
impl RequestableItem<LoginParams> for AuthResult {
    async fn request(params: LoginParams) -> Result<Request, Error> {
        Ok(Request::post("https://dummyjson.com/auth/login")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).map_err(|e| Error::SerdeError(e))?))
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}
