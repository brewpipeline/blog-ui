use yew::prelude::*;

use crate::components::ai_chat::AiChat;

#[function_component(AiChatPage)]
pub fn ai_chat_page() -> Html {
    html! {
        <div class="container my-4">
            <AiChat />
        </div>
    }
}
