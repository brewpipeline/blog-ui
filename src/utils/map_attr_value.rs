pub trait MapAttrValue {
    fn map_attr_value<F>(self, selectors: &'static str, attr: &'static str, transform: F) -> String
    where
        F: Fn(&str) -> String;
}

impl MapAttrValue for String {
    fn map_attr_value<F>(self, selectors: &'static str, attr: &'static str, transform: F) -> String
    where
        F: Fn(&str) -> String,
    {
        let fragment = scraper::Html::parse_fragment(self.as_ref());
        let Ok(selector) = scraper::Selector::parse(selectors) else {
            return self;
        };

        let mut result = self;

        for element in fragment.select(&selector) {
            if let Some(value) = element.value().attr(attr) {
                result = result.replace(value, transform(value).as_str());
            }
        }

        result
    }
}
