use chrono::Locale;

pub const DATE_LOCALE: Locale = Locale::en_US;

// Common
pub const COMMON_PAGE_NOT_FOUND_TITLE: &str = "Page not found";
pub const COMMON_PAGE_NOT_FOUND_TEXT: &str = "Page not found!";
pub const COMMON_LOADING: &str = "Loading...";
pub const COMMON_SAVE: &str = "Save";
pub const COMMON_DELETE: &str = "Delete";
pub const COMMON_SEND: &str = "Send";

// Navigation
pub const NAV_POSTS: &str = "Posts";
pub const NAV_AUTHORS: &str = "Authors";
pub const NAV_NEW_POST: &str = "New post";
pub const NAV_UNPUBLISHED: &str = "Unpublished";
pub const NAV_HIDDEN: &str = "Hidden";
pub const NAV_GO_TO_CHANNEL: &str = "Go to channel";

// Footer
pub const FOOTER_MENU: &str = "Menu";
pub const FOOTER_FEED: &str = "Feed";
pub const FOOTER_FEED_ARIA: &str = "Feed";
pub const FOOTER_INFO: &str = "Info";

// Auth
pub const AUTH_LOGIN: &str = "Log in";
pub const AUTH_LOGIN_TITLE: &str = "Log in";
pub const AUTH_LOGIN_ARIA: &str = "Log in";
pub const AUTH_LOGIN_VIA_TELEGRAM_TITLE: &str = "Log in via Telegram";
pub const AUTH_LOGIN_VIA_TELEGRAM_ARIA: &str = "Log in via Telegram";
pub const AUTH_LOGIN_ERROR: &str = "Authorization error: ";
pub const AUTH_USERNAME: &str = "Username";
pub const AUTH_PASSWORD: &str = "Password";
pub const AUTH_AUTHORIZED: &str = "Authorized!";
pub const AUTH_OR: &str = "OR";
pub const AUTH_LOGOUT_TITLE: &str = "Log out";
pub const AUTH_LOGOUT_CONFIRM: &str = "Are you sure you want to log out?";
pub const AUTH_LOGOUT: &str = "Log out";
pub const AUTH_NOT_AUTHORIZED: &str = "Not authorized!";
pub const AUTH_PROFILE: &str = "Profile";
pub const AUTH_MY_UNPUBLISHED: &str = "Unpublished";
pub const AUTH_SETTINGS: &str = "Settings";
pub const AUTH_LOGOUT_MENU: &str = "Log out";

// Posts page
pub const POSTS_TITLE: &str = "Posts";
pub const POSTS_ERROR: &str = "Failed to load posts!";
pub const POSTS_EMPTY: &str = "No posts.";

// Post page
pub const POST_TITLE: &str = "Post";
pub const POST_LINK_BROKEN_TITLE: &str = "Post link is broken";
pub const POST_LINK_BROKEN_TEXT: &str = "Post link is broken!";
pub const POST_ERROR_TITLE: &str = "Failed to load post";
pub const POST_ERROR_TEXT: &str = "Failed to load post!";
pub fn post_meta_title(title: &str) -> String {
    format!("{} - Post", title)
}

// Authors page
pub const AUTHORS_TITLE: &str = "Authors";
pub const AUTHORS_ERROR: &str = "Failed to load authors!";
pub const AUTHORS_EMPTY: &str = "No authors.";

// Author page
pub const AUTHOR_TITLE: &str = "Author";
pub const AUTHOR_POSTS: &str = "Author's posts ";
pub const AUTHOR_POSTS_ERROR: &str = "Failed to load author's posts!";
pub const AUTHOR_POSTS_EMPTY: &str = "Author has no posts.";
pub const AUTHOR_ERROR_TITLE: &str = "Failed to load author";
pub const AUTHOR_ERROR_TEXT: &str = "Failed to load author!";
pub fn author_meta_title(name: &str) -> String {
    format!("{} - Author", name)
}

// Author card
pub const AUTHOR_CARD_BAN: &str = " Ban ";
pub const AUTHOR_CARD_UNBAN: &str = " Unban ";
pub const AUTHOR_CARD_NAME_HIDDEN: &str = "(Name hidden)";
pub const AUTHOR_CARD_NAME_MISSING: &str = "(Name not specified)";
pub const AUTHOR_CARD_EDITOR: &str = "Editor-in-chief";
pub const AUTHOR_CARD_BLOCKED: &str = "Blocked";
pub const AUTHOR_CARD_STATUS_HIDDEN: &str = "(About info hidden)";
pub const AUTHOR_CARD_STATUS_MISSING: &str = "(No about info)";

// Search
pub const SEARCH_POSTS_PLACEHOLDER: &str = "Search posts...";
pub const SEARCH_AUTHORS_PLACEHOLDER: &str = "Search authors...";
pub const SEARCH_POSTS_TITLE: &str = "Search posts";
pub const SEARCH_AUTHORS_TITLE: &str = "Search authors";
pub const SEARCH_POSTS_ERROR: &str = "Failed to load post search results!";
pub const SEARCH_POSTS_EMPTY: &str = "No posts found!";
pub const SEARCH_POSTS_HINT: &str = "Start typing to search posts...";
pub const SEARCH_AUTHORS_ERROR: &str = "Failed to load author search results!";
pub const SEARCH_AUTHORS_EMPTY: &str = "No authors found!";
pub const SEARCH_AUTHORS_HINT: &str = "Start typing to search authors...";
pub const SEARCH_BUTTON_TITLE: &str = "Search";

// Tag page
pub const TAG_TITLE: &str = "Tag";
pub const TAG_PREFIX: &str = "Tag: ";
pub const TAG_LINK_BROKEN_TITLE: &str = "Tag link is broken";
pub const TAG_LINK_BROKEN_TEXT: &str = "Tag link is broken!";
pub const TAG_POSTS_ERROR: &str = "Failed to load posts by tag!";
pub const TAG_POSTS_EMPTY: &str = "No posts for this tag.";
pub const TAG_ERROR_TITLE: &str = "Failed to load tag";
pub const TAG_ERROR_TEXT: &str = "Failed to load tag!";
pub fn tag_meta_title(title: &str) -> String {
    format!("{} - Tag", title)
}

// Unpublished posts
pub const UNPUB_TITLE: &str = "Unpublished";
pub const UNPUB_AUTH_REQUIRED: &str = "Authorization required to view unpublished posts!";
pub const UNPUB_EDITORS_ONLY: &str = "Only editors can view unpublished posts!";
pub const UNPUB_ERROR: &str = "Failed to load unpublished posts!";
pub const UNPUB_EMPTY: &str = "No unpublished posts.";

// Hidden posts
pub const HIDDEN_TITLE: &str = "Hidden";
pub const HIDDEN_AUTH_REQUIRED: &str = "Authorization required to view hidden posts!";
pub const HIDDEN_EDITORS_ONLY: &str = "Only editors can view hidden posts!";
pub const HIDDEN_ERROR: &str = "Failed to load hidden posts!";
pub const HIDDEN_EMPTY: &str = "No hidden posts.";

// My unpublished posts
pub const MY_UNPUB_TITLE: &str = "My unpublished";
pub const MY_UNPUB_AUTH_REQUIRED: &str = "Authorization required to view your unpublished posts!";
pub const MY_UNPUB_ERROR: &str = "Failed to load your unpublished posts!";
pub const MY_UNPUB_EMPTY: &str = "No unpublished posts.";

// Edit post
pub const EDIT_POST_NEW_TITLE: &str = "New post";
pub const EDIT_POST_EDIT_TITLE: &str = "Edit post";
pub const EDIT_POST_NEW_AUTH: &str = "Only authorized authors can create posts!";
pub const EDIT_POST_EDIT_AUTH: &str = "Only authorized authors can edit posts!";
pub const EDIT_POST_DELETED: &str = "Post deleted!";
pub const EDIT_POST_EDITING: &str = "Editing post: ";
pub const EDIT_POST_IMAGE_LABEL: &str = "Image (URL) (Optional)";
pub const EDIT_POST_IMAGE_PLACEHOLDER: &str = "Something visually appealing...";
pub const EDIT_POST_TITLE_LABEL: &str = "Title";
pub const EDIT_POST_TITLE_PLACEHOLDER: &str = "Something attention-grabbing...";
pub const EDIT_POST_TITLE_VALIDATION: &str = "Please enter a post title, this field is required!";
pub const EDIT_POST_SUMMARY_LABEL: &str = "Short version";
pub const EDIT_POST_SUMMARY_PLACEHOLDER: &str = "Something short but important!";
pub const EDIT_POST_SUMMARY_VALIDATION: &str =
    "Please enter a short version of the post, this field is required!";
pub const EDIT_POST_CONTENT_LABEL: &str = "Full version (Optional)";
pub const EDIT_POST_CONTENT_PLACEHOLDER: &str = "Something long and bor... fun!";
pub const EDIT_POST_TAGS_LABEL: &str = "Tags (comma-separated) (Optional)";
pub const EDIT_POST_TAGS_PLACEHOLDER: &str = "Something reminiscent of...";
pub const EDIT_POST_UNPUBLISHED: &str = "Unpublished";
pub const EDIT_POST_PUBLISHED: &str = "Published";
pub const EDIT_POST_HIDDEN_STATUS: &str = "Hidden";
pub const EDIT_POST_BLOCKED: &str = "You are blocked!";
pub const EDIT_POST_ONLY_AUTHOR_OR_EDITOR: &str =
    "Only the author or an editor can edit this post!";
pub const EDIT_POST_LOADING: &str = "Loading post for editing...";
pub const EDIT_POST_LOAD_ERROR: &str = "Failed to load post for editing!";
pub fn edit_post_add_error(message: &str) -> String {
    format!("Error adding post: {}", message)
}
pub fn edit_post_edit_error(message: &str) -> String {
    format!("Error editing post: {}", message)
}
pub fn edit_post_delete_error(message: &str) -> String {
    format!("Error deleting post: {}", message)
}
pub const TINYMCE_LANG: &str = "en";

// Settings
pub const SETTINGS_TITLE: &str = "Settings";
pub const SETTINGS_AUTH_REQUIRED: &str = "Settings are only available to authorized authors!";
pub const SETTINGS_PRIMARY_TITLE: &str = "Primary profile data";
pub const SETTINGS_PRIMARY_RESET_TITLE: &str = "Reset primary data";
pub const SETTINGS_DATA_UPDATED: &str = "Data updated successfully: ";
pub const SETTINGS_DATA_ERROR: &str = "Error updating data: ";
pub const SETTINGS_USE_TELEGRAM: &str =
    "Use Telegram data (use the button below to select this option)";
pub const SETTINGS_SYNC_HINT: &str = "Also use the button to sync data.";
pub const SETTINGS_USE_CUSTOM: &str = "Use custom data";
pub const SETTINGS_SLUG: &str = "Profile name (unique)";
pub const SETTINGS_IMAGE_URL: &str = "Profile image (URL)";
pub const SETTINGS_FIRST_NAME: &str = "First name";
pub const SETTINGS_LAST_NAME: &str = "Last name";
pub const SETTINGS_SECONDARY_TITLE: &str = "Secondary profile data";
pub const SETTINGS_SECONDARY_RESET_TITLE: &str = "Reset secondary data";
pub const SETTINGS_ABOUT: &str = "About me";
pub const SETTINGS_EMAIL: &str = "Email";
pub const SETTINGS_PHONE: &str = "Phone";
pub const SETTINGS_BUTTON_NOT_READY: &str = "Button is still in development...";

// Comments
pub const COMMENT_TITLE: &str = "Comments";
pub const COMMENT_ERROR: &str = "Failed to load comments!";
pub const COMMENT_EMPTY: &str = "No comments.";
pub const COMMENT_PLACEHOLDER: &str = "Comment...";
pub const COMMENT_DELETE_TITLE: &str = "Delete comment";
pub const COMMENT_DELETING: &str = "Deleting...";
pub const COMMENT_DELETED: &str = "Deleted!";
pub const COMMENT_WAS_DELETED: &str = "Comment deleted.";

// Post card
pub const POSTCARD_EDIT_TITLE: &str = "Edit post";
pub const POSTCARD_UNPUBLISHED_TITLE: &str = "Unpublished";
pub const POSTCARD_HIDDEN_TITLE: &str = "Hidden";
pub const POSTCARD_STAR_ADD: &str = "Add to recommendations";
pub const POSTCARD_STAR_REMOVE: &str = "Remove from recommendations";

// ChatGPT
pub const CHATGPT_GREETING: &str = "Hi! I'm ChatGPT, adapted for this blog. I'm up to date with recent posts and can help you find something interesting. What would you like to read about?";
pub const CHATGPT_USER: &str = "You";
pub const CHATGPT_SYSTEM: &str = "System";
pub const CHATGPT_TYPING: &str = "Typing\u{2026}";
pub const CHATGPT_PLACEHOLDER: &str = "Ask something\u{2026}";
pub const CHATGPT_UNKNOWN_REASON: &str = "unknown reason";
pub const CHATGPT_NETWORK_ERROR: &str = "A network error occurred while receiving the response";
pub fn chatgpt_error(reason: &str) -> String {
    format!("An error occurred while receiving the response: {}", reason)
}

// Subscribe button
pub const SUBSCRIBE_SUBSCRIBED: &str = "You are subscribed to notifications";
pub const SUBSCRIBE_UNSUBSCRIBED: &str = "You are unsubscribed from notifications";

// Recommended post
pub const RECOMMENDED_TITLE: &str = "You might be interested";

// Body
pub const BODY_FEED_ARIA: &str = "Feed";
pub const BODY_FEED_TITLE: &str = "Feed";
pub const BODY_INFO_ARIA: &str = "Information";
pub const BODY_INFO_TITLE: &str = "Information";
pub const BODY_RULES: &str = "Rules";
pub const BODY_ABOUT: &str = "About Tikitko";
