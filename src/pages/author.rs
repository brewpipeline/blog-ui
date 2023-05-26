use yew::prelude::*;

use crate::components::item::*;
use crate::components::author_card::*;
use crate::content;

#[derive(PartialEq, Properties)]
pub struct AuthorProps {
    pub user_id: u64,
}

#[function_component(Author)]
pub fn author(props: &AuthorProps) -> Html {
    html! {
        <Item<content::User> 
            item_id={ props.user_id } 
            component={ |user| html! { <AuthorCard { user } /> } } 
        /> 
    }
}