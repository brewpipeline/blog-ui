use serde::Deserialize;

use crate::components::{list::ExternalListContainer, item::ExternalItem};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct UsersContainer {
    pub users: Vec<User>,
    pub total: u64,
    pub skip: u64,
    pub limit: u64,
}

impl ExternalListContainer for UsersContainer {
    fn url(limit: u64, skip: u64) -> String {
        let select = User::select();
        format!("https://dummyjson.com/users?limit={limit}&skip={skip}&select={select}")
    }
    type Item = User;
    fn items(self) -> Vec<Self::Item> {
        self.users
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct User {
    pub id: u64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "image")]
    pub image_url: String,
    pub gender: String,
    pub email: String
}

impl ExternalItem for User {
    fn url(id: u64) -> String {
        let select = Self::select();
        format!("https://dummyjson.com/users/{id}?select={select}")
    }
}

impl User {
    fn select() -> String {
        format!("id,firstName,lastName,image,gender,email")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct PostsContainer {
    pub posts: Vec<Post>,
    pub total: u64,
    pub skip: u64,
    pub limit: u64,
}

impl ExternalListContainer for PostsContainer {
    fn url(limit: u64, skip: u64) -> String {
        let select = Post::select();
        format!("https://dummyjson.com/posts?limit={limit}&skip={skip}&select={select}")
    }
    type Item = Post;
    fn items(self) -> Vec<Self::Item> {
        self.posts
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

impl ExternalItem for Post {
    fn url(id: u64) -> String {
        let select = Self::select();
        format!("https://dummyjson.com/posts/{id}?select={select}")
    }
}

impl Post {
    fn select() -> String {
        format!("id,title,body,userId,tags")
    }
}