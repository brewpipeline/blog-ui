use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
use crate::content;
use crate::utils::html_document;

#[derive(PartialEq, Properties)]
pub struct AuthorProps {
    pub user_id: u64,
}

#[function_component(Author)]
pub fn author(props: &AuthorProps) -> Html {
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Автор".to_string());
    html! {
        <Item<content::User>
            item_id={ props.user_id }
            component={ |user: Option<content::User>| {
                if let Some(user) = &user {
                    html_document::reset_title_and_meta();
                    html_document::set_prefix_default_title(format!("{} - Автор", user.username.clone()));
                }
                html! { <AuthorCard { user } link_to=false /> }
            } }
        />
    }
}
