pub use blog_generic::entities::*;
#[cfg(feature = "client")]
use gloo_net::http::{Request, Response};
#[cfg(feature = "client")]
use gloo_net::Error;
use serde::{Deserialize, Serialize};

use crate::utils::*;

//
// ExternalCodable impl
//
//

impl<T> ExternalCodable for T
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    fn encode(&self) -> Option<AppContent> {
        AppContent::json_encode(self)
    }
    fn decode(app_content: AppContent) -> Option<Self> {
        app_content.json_decode()
    }
}

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

#[derive(Clone, PartialEq)]
pub struct OptionTokened<P> {
    pub token: Option<String>,
    pub params: P,
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
            "{url}/api/authors/search/{query}?limit={limit}&offset={skip}",
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

#[derive(Clone, PartialEq)]
pub struct AuthorMeParams;

#[derive(Clone, PartialEq)]
pub struct BlockAuthorIdParams {
    pub id: u64,
    pub block: bool,
}

#[derive(Clone, PartialEq)]
pub struct SubscribeAuthorIdParams {
    pub id: u64,
    pub subscribe: bool,
}

#[derive(Clone, PartialEq)]
pub struct UpdateMinimalAuthor {
    pub update_minimal_author: CommonMinimalAuthor,
}

#[derive(Clone, PartialEq)]
pub struct UpdateSecondaryAuthor {
    pub update_secondary_author: CommonSecondaryAuthor,
}

#[derive(Clone, PartialEq)]
pub struct AuthorResetOverrideSocialData;

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<AuthorSlugParams> for API<AuthorContainer> {
    async fn request(params: AuthorSlugParams) -> Result<Request, Error> {
        let AuthorSlugParams { slug } = params;
        let url = format!("{url}/api/author/slug/{slug}", url = crate::API_URL);
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

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<BlockAuthorIdParams>> for API<()> {
    async fn request(params: Tokened<BlockAuthorIdParams>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: BlockAuthorIdParams { id, block },
        } = params;
        let url = format!(
            "{url}/api/author/id/{id}/{state}",
            url = crate::API_URL,
            state = if block { "block" } else { "unblock" }
        );
        Ok(Request::get(url.as_str())
            .header("Token", token.as_str())
            .build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<SubscribeAuthorIdParams>> for API<()> {
    async fn request(params: Tokened<SubscribeAuthorIdParams>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: SubscribeAuthorIdParams { id, subscribe },
        } = params;
        let url = format!(
            "{url}/api/author/id/{id}/{state}",
            url = crate::API_URL,
            state = if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            }
        );
        Ok(Request::patch(url.as_str())
            .header("Token", token.as_str())
            .build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<AuthorResetOverrideSocialData>> for API<()> {
    async fn request(params: Tokened<AuthorResetOverrideSocialData>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: AuthorResetOverrideSocialData,
        } = params;
        let url = format!(
            "{url}/api/author/reset_override_social_data",
            url = crate::API_URL
        );
        Ok(Request::patch(url.as_str())
            .header("Token", token.as_str())
            .build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<UpdateMinimalAuthor>> for API<()> {
    async fn request(params: Tokened<UpdateMinimalAuthor>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: UpdateMinimalAuthor {
                update_minimal_author,
            },
        } = params;
        let url = format!("{url}/api/author/minimal", url = crate::API_URL);
        Ok(Request::patch(url.as_str())
            .header("Token", token.as_str())
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(&update_minimal_author).map_err(|e| Error::SerdeError(e))?,
            )?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<UpdateSecondaryAuthor>> for API<()> {
    async fn request(params: Tokened<UpdateSecondaryAuthor>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: UpdateSecondaryAuthor {
                update_secondary_author,
            },
        } = params;
        let url = format!("{url}/api/author/secondary", url = crate::API_URL);
        Ok(Request::patch(url.as_str())
            .header("Token", token.as_str())
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(&update_secondary_author)
                    .map_err(|e| Error::SerdeError(e))?,
            )?)
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
pub struct PostsContainerParams;

#[derive(Clone, PartialEq)]
pub struct PostsContainerSearchParams {
    pub query: String,
}

#[derive(Clone, PartialEq)]
pub struct PostsContainerAuthorParams {
    pub author_id: u64,
}

#[derive(Clone, PartialEq)]
pub struct PostsContainerTagParams {
    pub tag_id: u64,
}

#[derive(Clone, PartialEq)]
pub struct UnpublishedPostsContainerParams;

#[derive(Clone, PartialEq)]
pub struct UnpublishedPostsContainerAuthorParams {
    pub author_id: u64,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<PostsContainerParams>> for API<PostsContainer> {
    async fn request(
        params: ExternalListContainerParams<PostsContainerParams>,
    ) -> Result<Request, Error> {
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
impl RequestableItem<ExternalListContainerParams<PostsContainerSearchParams>>
    for API<PostsContainer>
{
    async fn request(
        params: ExternalListContainerParams<PostsContainerSearchParams>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            limit,
            skip,
            params: PostsContainerSearchParams { query },
        } = params;
        let url = format!(
            "{url}/api/posts/search/{query}?limit={limit}&offset={skip}",
            url = crate::API_URL,
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<PostsContainerAuthorParams>>
    for API<PostsContainer>
{
    async fn request(
        params: ExternalListContainerParams<PostsContainerAuthorParams>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            limit,
            skip,
            params: PostsContainerAuthorParams { author_id },
        } = params;
        let url = format!(
            "{url}/api/posts/author/id/{author_id}?limit={limit}&offset={skip}",
            url = crate::API_URL,
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<PostsContainerTagParams>> for API<PostsContainer> {
    async fn request(
        params: ExternalListContainerParams<PostsContainerTagParams>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            limit,
            skip,
            params: PostsContainerTagParams { tag_id },
        } = params;
        let url = format!(
            "{url}/api/posts/tag/{tag_id}?limit={limit}&offset={skip}",
            url = crate::API_URL,
        );
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<ExternalListContainerParams<OptionTokened<UnpublishedPostsContainerParams>>>
    for API<PostsContainer>
{
    async fn request(
        params: ExternalListContainerParams<OptionTokened<UnpublishedPostsContainerParams>>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            params: OptionTokened { token, .. },
            limit,
            skip,
        } = params;
        let url = format!(
            "{url}/api/posts/unpublished?limit={limit}&offset={skip}",
            url = crate::API_URL
        );
        let mut request = Request::get(url.as_str());
        if let Some(token) = token {
            request = request.header("Token", token.as_str());
        }
        Ok(request.build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl
    RequestableItem<
        ExternalListContainerParams<OptionTokened<UnpublishedPostsContainerAuthorParams>>,
    > for API<PostsContainer>
{
    async fn request(
        params: ExternalListContainerParams<OptionTokened<UnpublishedPostsContainerAuthorParams>>,
    ) -> Result<Request, Error> {
        let ExternalListContainerParams {
            params:
                OptionTokened {
                    token,
                    params: UnpublishedPostsContainerAuthorParams { author_id },
                },
            limit,
            skip,
        } = params;
        let url = format!(
            "{url}/api/posts/unpublished/author/id/{author_id}?limit={limit}&offset={skip}",
            url = crate::API_URL
        );
        let mut request = Request::get(url.as_str());
        if let Some(token) = token {
            request = request.header("Token", token.as_str());
        }
        Ok(request.build()?)
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
pub struct PostParams {
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

#[derive(Clone, PartialEq)]
pub struct DeletePostParams {
    pub id: u64,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<OptionTokened<PostParams>> for API<PostContainer> {
    async fn request(params: OptionTokened<PostParams>) -> Result<Request, Error> {
        let OptionTokened {
            token,
            params: PostParams { id },
        } = params;
        let url = format!("{url}/api/post/{id}", url = crate::API_URL);
        let mut request = Request::get(url.as_str());
        if let Some(token) = token {
            request = request.header("Token", token.as_str())
        }
        Ok(request.build()?)
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

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<DeletePostParams>> for API<()> {
    async fn request(params: Tokened<DeletePostParams>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: DeletePostParams { id },
        } = params;
        let url = format!("{url}/api/post/{id}", url = crate::API_URL);
        Ok(Request::delete(url.as_str())
            .header("Token", token.as_str())
            .build()?)
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
// Tag
//
//

#[derive(Clone, PartialEq)]
pub struct TagIdParams {
    pub id: u64,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<TagIdParams> for API<TagContainer> {
    async fn request(params: TagIdParams) -> Result<Request, Error> {
        let TagIdParams { id } = params;
        let url = format!("{url}/api/tag/{id}", url = crate::API_URL);
        Ok(Request::get(url.as_str()).build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

impl ExternalItemContainer for TagContainer {
    type Item = Tag;
    fn item(self) -> Self::Item {
        self.tag
    }
}

//
// Comments
//
//

#[derive(Clone, PartialEq)]
pub struct CommentsContainerPostIdParams {
    pub post_id: u64,
    pub request_index: u64,
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
            params:
                CommentsContainerPostIdParams {
                    post_id,
                    request_index,
                },
            limit,
            skip,
        } = params;
        let url = format!(
            "{url}/api/comments/{post_id}?limit={limit}&offset={skip}&request_index={request_index}",
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
// Comment
//
//

#[derive(Clone, PartialEq)]
pub struct CreateCommentParams {
    pub comment: CommonComment,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<CreateCommentParams>> for API<()> {
    async fn request(params: Tokened<CreateCommentParams>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: CreateCommentParams { comment },
        } = params;
        let url = format!("{url}/api/comment", url = crate::API_URL,);
        Ok(Request::post(url.as_str())
            .header("Token", token.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&comment).map_err(|e| Error::SerdeError(e))?)?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[derive(Clone, PartialEq)]
pub struct DeleteCommentParams {
    pub comment_id: u64,
}

#[cfg(feature = "client")]
#[async_trait(?Send)]
impl RequestableItem<Tokened<DeleteCommentParams>> for API<()> {
    async fn request(params: Tokened<DeleteCommentParams>) -> Result<Request, Error> {
        let Tokened {
            token,
            params: DeleteCommentParams { comment_id },
        } = params;
        let url = format!("{url}/api/comment/{comment_id}", url = crate::API_URL,);
        Ok(Request::delete(url.as_str())
            .header("Token", token.as_str())
            .build()?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
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

#[cfg(all(feature = "client", feature = "yandex"))]
#[async_trait(?Send)]
impl RequestableItem<LoginYandexQuestion> for API<LoginAnswer> {
    async fn request(params: LoginYandexQuestion) -> Result<Request, Error> {
        let url = format!("{url}/api/ylogin", url = crate::API_URL);
        Ok(Request::post(url.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).map_err(|e| Error::SerdeError(e))?)?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}

#[cfg(all(feature = "client", feature = "telegram"))]
#[async_trait(?Send)]
impl RequestableItem<LoginTelegramQuestion> for API<LoginAnswer> {
    async fn request(params: LoginTelegramQuestion) -> Result<Request, Error> {
        let url = format!("{url}/api/tlogin", url = crate::API_URL);
        Ok(Request::post(url.as_str())
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&params).map_err(|e| Error::SerdeError(e))?)?)
    }
    async fn response(response: Response) -> Result<Self, Error> {
        response.json().await
    }
}
