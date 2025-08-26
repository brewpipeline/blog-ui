use yew::prelude::*;

#[cfg(feature = "client")]
use crate::content;
#[cfg(feature = "client")]
use crate::utils::external::ExternalResultContainer;
#[cfg(feature = "client")]
use blog_generic::entities::{ChatAnswer, ChatQuestion};
#[cfg(feature = "client")]
use gloo::storage::{LocalStorage, Storage};
#[cfg(feature = "client")]
use gloo_net::http::Request;
#[cfg(feature = "client")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "client")]
use wasm_bindgen_futures::spawn_local;

use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "client", derive(Serialize, Deserialize))]
struct ChatMessage {
    from_user: bool,
    pending: bool,
    text: String,
}

impl ChatMessage {
    fn ai(text: impl Into<String>) -> Self {
        Self {
            from_user: false,
            pending: false,
            text: text.into(),
        }
    }

    fn user_pending(text: impl Into<String>) -> Self {
        Self {
            from_user: true,
            pending: true,
            text: text.into(),
        }
    }
}

fn render_message(msg: &ChatMessage) -> Html {
    let alignment = if msg.from_user {
        "justify-content-end"
    } else {
        "justify-content-start"
    };
    let class = if msg.from_user {
        "chat-message user"
    } else {
        "chat-message ai"
    };
    let icon = if msg.from_user {
        "bi-person-circle"
    } else {
        "bi-robot"
    };

    html! {
        <div class={classes!("d-flex", alignment)}>
            <div class={class}>
                <i class={classes!("bi", icon)}></i>
                <span>{ &msg.text }</span>
            </div>
        </div>
    }
}

fn latest_user_question(messages: &[ChatMessage]) -> String {
    messages
        .iter()
        .rev()
        .find(|m| m.from_user)
        .map(|m| m.text.clone())
        .unwrap_or_default()
}

fn pending_question(messages: &[ChatMessage]) -> String {
    messages
        .last()
        .filter(|m| m.from_user && m.pending)
        .map(|m| m.text.clone())
        .unwrap_or_default()
}

const GREETING: &str = "Привет! Что хотите почитать?";

#[function_component(AiChat)]
pub fn ai_chat() -> Html {
    #[cfg(feature = "client")]
    const STORAGE_KEY: &str = "ai-chat";

    let chat = use_state(|| {
        #[cfg(feature = "client")]
        {
            let mut history: Vec<ChatMessage> = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
            if history.is_empty() {
                history.push(ChatMessage::ai(GREETING));
            }
            history
        }
        #[cfg(not(feature = "client"))]
        {
            vec![ChatMessage::ai(GREETING)]
        }
    });

    let sending = use_state(|| false);
    let expanded = use_state(|| false);

    #[cfg(feature = "client")]
    {
        let chat_value = (*chat).clone();
        use_effect_with(chat_value, move |msgs: &Vec<ChatMessage>| {
            LocalStorage::set(STORAGE_KEY, msgs).ok();
            || ()
        });
    }

    let oninput = {
        let chat = chat.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            chat.set({
                let mut msgs = (*chat).clone();
                match msgs.last_mut() {
                    Some(msg) if msg.from_user && msg.pending => msg.text = value,
                    _ => msgs.push(ChatMessage::user_pending(value)),
                }
                msgs
            });
        })
    };

    let send = {
        let chat = chat.clone();
        let sending = sending.clone();
        Callback::from(move |_| {
            if *sending {
                return;
            }
            let question = pending_question(&chat).trim().to_string();
            if question.is_empty() {
                return;
            }

            chat.set({
                let mut msgs = (*chat).clone();
                if let Some(last) = msgs.last_mut() {
                    last.pending = false;
                }
                msgs
            });
            sending.set(true);

            #[cfg(feature = "client")]
            {
                let chat = chat.clone();
                let sending = sending.clone();
                spawn_local(async move {
                    let url = format!("{}/chat", crate::API_URL);
                    let body = serde_json::to_string(&ChatQuestion { question }).unwrap();
                    let resp = Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(body)
                        .unwrap()
                        .send()
                        .await;

                    if let Ok(resp) = resp {
                        if let Ok(api) = resp.json::<content::API<ChatAnswer>>().await {
                            if let Ok(answer) = api.result() {
                                chat.set({
                                    let mut msgs = (*chat).clone();
                                    msgs.push(ChatMessage::ai(answer.answer));
                                    msgs
                                });
                            }
                        }
                    }
                    sending.set(false);
                });
            }
        })
    };

    let open_chat = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(true))
    };

    let close_chat = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(false))
    };

    let onclick = {
        let send = send.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            send.emit(());
        })
    };

    let onkeydown = {
        let send = send.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                send.emit(());
            }
        })
    };

    let container_class = classes!(
        "ai-chat",
        "mb-3",
        "w-100",
        if *expanded { "expanded" } else { "" }
    );

    html! {
        <div class={container_class}>
            <div class="collapsed" onclick={open_chat}>
                <div class="input-group">
                    <span class="input-group-text"><i class="bi bi-robot"></i></span>
                    <input
                        class="form-control"
                        placeholder="Ask what to read"
                        value={latest_user_question(&chat)}
                        readonly=true
                    />
                </div>
            </div>
            <div class="chat card">
                <div class="card-header d-flex justify-content-between align-items-center">
                    { "AI рекомендации" }
                    <button type="button" class="btn-close" aria-label="Close" onclick={close_chat}></button>
                </div>
                <div class="chat-body card-body">
                    { for (*chat).iter().filter(|m| !m.pending).map(render_message) }
                </div>
                <div class="card-footer d-flex">
                    <input
                        class="form-control me-2"
                        value={pending_question(&chat)}
                        {oninput}
                        {onkeydown}
                    />
                    <button class="btn btn-purple" {onclick} disabled={*sending}>{ "Отправить" }</button>
                </div>
            </div>
        </div>
    }
}
