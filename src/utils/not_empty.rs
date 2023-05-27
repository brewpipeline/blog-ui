pub fn not_empty(value: Option<String>) -> Option<String> {
    let Some(value) = value else {
        return None
    };
    if value.is_empty() {
        return None;
    }
    Some(value)
}
