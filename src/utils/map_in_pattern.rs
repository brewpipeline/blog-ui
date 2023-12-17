pub trait MapInPattern {
    fn map_in_pattern<F>(self, pattern: [&str; 2], transform: F) -> String
    where
        F: Fn(&str) -> String;
}

impl<T: AsRef<str>> MapInPattern for T {
    fn map_in_pattern<F>(self, pattern: [&str; 2], transform: F) -> String
    where
        F: Fn(&str) -> String,
    {
        let str: &str = self.as_ref();
        let mut result = String::new();
        let mut start = 0;

        while let Some(start_index) = str[start..].find(pattern[0]) {
            let inner_start = start + start_index + pattern[0].len();

            let Some(inner_len) = str[inner_start..].find(pattern[1]) else {
                break;
            };

            let inner_end = inner_start + inner_len;
            let end = inner_end + pattern[1].len();

            result.push_str(&str[start..inner_start]);
            result.push_str(&transform(&str[inner_start..inner_end]));
            result.push_str(&str[inner_end..end]);

            start = end;
        }

        result.push_str(&str[start..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let html = r#"<img src="old_url.jpg">"#;
        let result = html.map_in_pattern(["src=\"", "\""], |url| {
            assert_eq!(url, "old_url.jpg");
            "new_url.jpg".to_string()
        });
        assert_eq!(result, r#"<img src="new_url.jpg">"#);
    }

    #[test]
    fn test_multiple_attributes() {
        let html = r#"<img src="old.jpg"><img src="another_old.jpg">"#;
        let result = html.map_in_pattern(["src=\"", "\""], |url| {
            if url == "old.jpg" {
                "new1.jpg".to_string()
            } else if url == "another_old.jpg" {
                "new2.jpg".to_string()
            } else {
                unreachable!()
            }
        });
        assert_eq!(result, r#"<img src="new1.jpg"><img src="new2.jpg">"#);
    }

    #[test]
    fn test_no_matching_attributes() {
        let html = r#"<div class="class">Content</div>"#;
        let result = html.map_in_pattern(["src=\"", "\""], |url| unreachable!());
        assert_eq!(result, html);
    }

    #[test]
    fn test_nested_attributes() {
        let html = r#"<div src="outer"><img src="inner.jpg"></div>"#;
        let result = html.map_in_pattern(["src=\"", "\""], |url| {
            url.replace("inner.jpg", "changed.jpg")
        });
        assert_eq!(result, r#"<div src="outer"><img src="changed.jpg"></div>"#);
    }
}
