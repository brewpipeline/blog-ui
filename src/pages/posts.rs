use yew::prelude::*;

use crate::components::list::*;
use crate::components::post_card::*;
use crate::content;

use crate::Route;

#[function_component(Posts)]
pub fn posts() -> Html {
    html! {
        <List<content::PostsContainer>
            params={ () }
            route_to_page={ Route::Posts } 
            component={ |post| html! { <PostCard { post } /> } } 
        /> 
    }
}