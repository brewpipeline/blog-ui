use blog_generic::*;

use crate::content::*;

pub fn author_slug_formatter(author: &Author) -> String {
    if author.blocked == 1 {
        format!("blocked_id_{id}", id = author.id)
    } else {
        author_slug_utils::clean(&author.slug)
    }
}
