use std::fmt::Display;

pub fn profile_image<S: Display>(seed: &S) -> String {
    format!("https://api.dicebear.com/7.x/thumbs/svg?seed={seed}")
}
