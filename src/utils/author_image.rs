use std::fmt::Display;

use blog_generic::entities::Author;

fn profile_image<S: Display>(seed: &S) -> String {
    format!("https://api.dicebear.com/7.x/bottts/svg?seed={seed}&baseColor=f2f2f2&eyes=dizzy,eva,frame1,frame2,happy,hearts,robocop,round,roundFrame01,roundFrame02,sensor,shade01")
}

pub fn author_image(author: &Author) -> String {
    author
        .image_url
        .clone()
        .unwrap_or_else(|| profile_image(&author.slug))
}
