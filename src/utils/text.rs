use yew::prelude::*;

pub fn render_text_with_newlines(text: &str) -> Html {
    let mut nodes: Vec<Html> = Vec::new();
    let mut first = true;
    for line in text.split('\n') {
        if !first {
            nodes.push(html! { <br/> });
        }
        nodes.push(html! { { line } });
        first = false;
    }
    html! { for nodes }
}
