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
// Token
//
//

#[derive(Clone, PartialEq)]
pub struct TokenParam<D> {
    pub token: String,
    pub data: D,
}

//
// AuthorImageUrl
//
//

pub fn author_image_url(slug: &String) -> String {
    format!(
        "https://api.dicebear.com/6.x/bottts-neutral/svg?seed={}",
        slug,
    )
}

//
// Tag
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub title: String,
    pub slug: String,
}

//
// ShortAuthor
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortAuthor {
    pub slug: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl ShortAuthor {
    pub fn image_url(&self) -> String {
        author_image_url(&self.slug)
    }
}

//
// Authors
//
//

#[derive(Clone, PartialEq)]
pub struct AuthorsContainerSearchParam {
    pub query: String,
}

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

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<AuthorsContainerSearchParam>>
    for API<AuthorsContainer>
{
    async fn request(
        params: ExternalListContainerParams<AuthorsContainerSearchParam>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            limit,
            skip,
            params,
        } = params;
        let url = format!(
            "http://127.0.0.1:3000/api/search/authors/{query}?limit={limit}&offset={skip}",
            query = params.query,
        );
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

impl Author {
    pub fn image_url(&self) -> String {
        author_image_url(&self.slug)
    }
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

#[async_trait(?Send)]
impl RequestableItem<TokenParam<()>> for API<AuthorContainer> {
    async fn request(params: TokenParam<()>) -> Result<Request, Error> {
        let TokenParam { token, data: _ } = params;
        let url = format!("http://127.0.0.1:3000/api/author/me");
        Ok(Request::get(url.as_str()).header("Token", token.as_str()))
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
// NewPost
//
//

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPost {
    pub title: String,
    pub slug: String, // KONCH
    pub published: u64,
    pub summary: String,
    pub content: Option<String>,
    pub tags: Vec<String>,
}

#[async_trait(?Send)]
impl RequestableItem<TokenParam<NewPost>> for API<PostContainer> {
    async fn request(params: TokenParam<NewPost>) -> Result<Request, Error> {
        let TokenParam {
            token,
            data: new_post,
        } = params;
        Ok(Request::post("http://127.0.0.1:3000/api/post")
            .header("Token", token.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&new_post).map_err(|e| Error::SerdeError(e))?))
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
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
        let url = format!("http://127.0.0.1:3000/api/posts?limit={limit}&offset={skip}");
        Ok(Request::get(url.as_str()))
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

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
            params,
        } = params;
        let url = format!(
            "http://127.0.0.1:3000/api/search/posts/{query}?limit={limit}&offset={skip}",
            query = params.query,
        );
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
            self.tags
                .clone()
                .into_iter()
                .map(|v| v.title)
                .collect::<Vec<String>>()
                .join(","),
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
// Comments
//
//

#[derive(Clone, PartialEq)]
pub struct CommentsContainerPostSlugParam {
    pub post_slug: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct CommentsContainer {
    pub comments: Vec<Comment>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<CommentsContainerPostSlugParam>>
    for API<CommentsContainer>
{
    async fn request(
        params: ExternalListContainerParams<CommentsContainerPostSlugParam>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            params,
            limit,
            skip,
        } = params;
        let url = format!(
            "http://127.0.0.1:3000/api/comments/{post_slug}?limit={limit}&offset={skip}",
            post_slug = params.post_slug,
        );
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
        self.offset
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub post_id: i64,
    pub created_at: i64,
    pub content: String,
    pub short_author: ShortAuthor,
}

//
// Auth
//
//

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AuthParams {
    pub slug: String,
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TokenContainer {
    pub token: String,
}

#[async_trait(?Send)]
impl RequestableItem<AuthParams> for API<TokenContainer> {
    async fn request(params: AuthParams) -> Result<Request, Error> {
        Ok(Request::post("http://127.0.0.1:3000/api/login")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).map_err(|e| Error::SerdeError(e))?))
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}
