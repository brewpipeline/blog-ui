use yew::prelude::*;

use crate::components::list::*;
use crate::components::author_card::*;
use crate::content;

use crate::Route;

#[function_component(Authors)]
pub fn authors() -> Html {
    html! {
        <List<content::UsersContainer>
            params={ () }
            route_to_page={ Route::Authors } 
            component={ |user| html! { <AuthorCard { user } /> } } 
        /> 
    }
}