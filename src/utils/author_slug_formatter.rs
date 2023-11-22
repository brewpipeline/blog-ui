use blog_generic::entities::Author;

pub fn author_slug_formatter(author: &Author) -> String {
    if author.blocked == 1 {
        format!("blocked_id_{id}", id = author.id)
    } else {
        blog_generic::clean_author_slug(&author.slug)
    }
}
