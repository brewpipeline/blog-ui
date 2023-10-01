use std::fmt::Display;

use blog_generic::entities::Author;

fn profile_image<S: Display>(seed: &S) -> String {
    format!("https://api.dicebear.com/7.x/thumbs/svg?seed={seed}")
}

pub fn author_image(author: &Author) -> String {
    author
        .image_url
        .clone()
        .unwrap_or_else(|| profile_image(&author.slug))
}
