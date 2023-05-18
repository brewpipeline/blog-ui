use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct UsersContainer {
    pub users: Vec<User>,
    pub total: u64,
    pub skip: u64,
    pub limit: u64,
}

impl UsersContainer {
    pub fn url(limit: u64, skip: u64) -> String {
        let select = User::select();
        format!("https://dummyjson.com/users?limit={limit}&skip={skip}&select={select}")
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

impl User {
    pub fn url(id: u64) -> String {
        let select = Self::select();
        format!("https://dummyjson.com/users/{id}?select={select}")
    }
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

impl PostsContainer {
    pub fn url(limit: u64, skip: u64) -> String {
        let select = Post::select();
        format!("https://dummyjson.com/posts?limit={limit}&skip={skip}&select={select}")
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
    pub fn url(id: u64) -> String {
        let select = Self::select();
        format!("https://dummyjson.com/posts/{id}?select={select}")
    }
    fn select() -> String {
        format!("id,title,body,userId,tags")
    }
}