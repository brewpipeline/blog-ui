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

impl<T: Identifiable> Identifiable for TokenParam<T> {
    type Id = T::Id;
    fn id(&self) -> &Self::Id {
        if core::any::type_name::<T>() == core::any::type_name::<TokenParam<T>>() {
            panic!("recursion")
        }
        self.id()
    }
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
    pub id: u64,
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
            params: AuthorsContainerSearchParam { query },
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

impl Identifiable for AuthorSlugParam {
    type Id = String;
    fn id(&self) -> &Self::Id {
        &self.slug
    }
}

#[derive(Clone, PartialEq)]
pub struct AuthorMeParam;

impl Identifiable for AuthorMeParam {
    type Id = String;
    fn id(&self) -> &Self::Id {
        unreachable!()
    }
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

impl Identifiable for Author {
    type Id = String;
    fn id(&self) -> &Self::Id {
        &self.slug
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
        let url = format!("{url}/api/author/{slug}", url = crate::API_URL);
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[async_trait(?Send)]
impl RequestableItem<TokenParam<AuthorMeParam>> for API<AuthorContainer> {
    async fn request(params: TokenParam<AuthorMeParam>) -> Result<Request, Error> {
        let TokenParam { token, data: _ } = params;
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
// NewPost
//
//

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPost {
    pub title: String,
    pub published: u8,
    pub summary: String,
    pub content: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPostContainer {
    pub created_post: Post,
}

#[async_trait(?Send)]
impl RequestableItem<TokenParam<NewPost>> for API<NewPostContainer> {
    async fn request(params: TokenParam<NewPost>) -> Result<Request, Error> {
        let TokenParam {
            token,
            data: new_post,
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

impl ExternalItemContainer for NewPostContainer {
    type Item = Post;
    fn item(self) -> Self::Item {
        self.created_post
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
pub struct PostIdParam {
    pub id: u64,
}

impl Identifiable for PostIdParam {
    type Id = u64;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: u64,
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

impl Identifiable for Post {
    type Id = u64;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContainer {
    pub post: Post,
}

#[async_trait(?Send)]
impl RequestableItem<PostIdParam> for API<PostContainer> {
    async fn request(params: PostIdParam) -> Result<Request, Error> {
        let PostIdParam { id } = params;
        let url = format!("{url}/api/post/{id}", url = crate::API_URL);
        Ok(Request::get(url.as_str()).build()?)
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
pub struct CommentsContainerPostIdParam {
    pub post_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct CommentsContainer {
    pub comments: Vec<Comment>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<CommentsContainerPostIdParam>>
    for API<CommentsContainer>
{
    async fn request(
        params: ExternalListContainerParams<CommentsContainerPostIdParam>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            params: CommentsContainerPostIdParam { post_id },
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
        let url = format!("{url}/api/login", url = crate::API_URL);
        Ok(Request::post(url.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).map_err(|e| Error::SerdeError(e))?)?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}
