use gloo_net::Error;
use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::components::list::*;
use crate::components::item::*;
use crate::get::*;
use crate::hash_map_context::*;

//
// UsersContainer
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct UsersContainer {
    pub users: Vec<User>,
    pub total: u64,
    pub skip: u64,
    pub limit: u64,
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for UsersContainer {
    fn request(params: ExternalListContainerParams<()>) -> Request {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let select = User::select();
        let url = format!("https://dummyjson.com/users?limit={limit}&skip={skip}&select={select}");
        Request::get(url.as_str())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
} 

impl ExternalListContainer for UsersContainer {
    type Item = User;
    fn items(&self) -> Vec<Self::Item> {
        self.users.clone()
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

//
// User
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct User {
    pub id: u64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "image")]
    pub image_url: String,
    pub username: String,
    pub email: String,
}

impl KeyedItem for User {
    type Key = u64;
    fn key(&self) -> Self::Key {
        self.id
    }
}

#[async_trait(?Send)]
impl RequestableItem<ExternalItemParams> for User {
    fn request(params: ExternalItemParams) -> Request {
        let select = Self::select();
        let url = format!("https://dummyjson.com/users/{id}?select={select}", id = params.id);
        Request::get(url.as_str())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
} 

impl ExternalItem for User {}

impl User {
    fn select() -> String {
        format!("id,firstName,lastName,image,username,email")
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
    pub skip: u64,
    pub limit: u64,
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for PostsContainer {
    fn request(params: ExternalListContainerParams<()>) -> Request {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let select = Post::select();
        let url = format!("https://dummyjson.com/posts?limit={limit}&skip={skip}&select={select}");
        Request::get(url.as_str())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
} 

impl ExternalListContainer for PostsContainer {
    type Item = Post;
    fn items(&self) -> Vec<Self::Item> {
        self.posts.clone()
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

//
// Post
//
//

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub tags: Vec<String>,
}

impl Post {
    pub fn image_url(&self) -> String {
        format!(
            "https://source.unsplash.com/random/{}x{}?{}&sig={}",
            400,
            100,
            self.tags.join(","),
            self.id,
        )
    }
}

impl KeyedItem for Post {
    type Key = u64;
    fn key(&self) -> Self::Key {
        self.id
    }
}

#[async_trait(?Send)]
impl RequestableItem<ExternalItemParams> for Post {
    fn request(params: ExternalItemParams) -> Request {
        let select = Self::select();
        let url = format!("https://dummyjson.com/posts/{id}?select={select}", id = params.id);
        Request::get(url.as_str())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
} 

impl ExternalItem for Post {}

impl Post {
    fn select() -> String {
        format!("id,title,body,userId,tags")
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
impl RequestableItem<ExternalListContainerParams<CommentsContainerPostIdParam>> for CommentsContainer {
    fn request(params: ExternalListContainerParams<CommentsContainerPostIdParam>) -> Request {
        let ExternalListContainerParams { custom_params, limit, skip } = params;
        let url = format!("https://dummyjson.com/comments/post/{post_id}?limit={limit}&skip={skip}", post_id = custom_params.post_id);
        Request::get(url.as_str())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<()>> for CommentsContainer {
    fn request(params: ExternalListContainerParams<()>) -> Request {
        let ExternalListContainerParams { limit, skip, .. } = params;
        let url = format!("https://dummyjson.com/comments?limit={limit}&skip={skip}");
        Request::get(url.as_str())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
} 

impl ExternalListContainer for CommentsContainer {
    type Item = Comment;
    fn items(&self) -> Vec<Self::Item> {
        self.comments.clone()
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

//
// Comment
//
//

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

impl KeyedItem for Comment {
    type Key = u64;
    fn key(&self) -> Self::Key {
        self.id
    }
}

#[async_trait(?Send)]
impl RequestableItem<ExternalItemParams> for Comment {
    fn request(params: ExternalItemParams) -> Request {
        let url = format!("https://dummyjson.com/comments/{id}", id = params.id);
        Request::get(url.as_str())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
} 

impl ExternalItem for Comment {}

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
    fn request(params: LoginParams) -> Request {
        Request::post("https://dummyjson.com/auth/login")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).unwrap())
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
} 