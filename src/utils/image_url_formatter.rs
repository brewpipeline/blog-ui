use base64::engine::general_purpose;
use base64::Engine;

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Normal,
    Small,
    Medium,
}

impl ImageType {
    fn path_part(&self) -> &'static str {
        match self {
            ImageType::Normal => "",
            ImageType::Small => "small/",
            ImageType::Medium => "medium/",
        }
    }
}

pub fn image_url_formatter<S: AsRef<str>>(image_type: ImageType, image_url: S) -> String {
    format!(
        "/images/external/mirror/{path_part}{base64_image_url}", // TODO: move base url to env var
        path_part = image_type.path_part(),
        base64_image_url = general_purpose::URL_SAFE.encode(image_url.as_ref()),
    )
}
