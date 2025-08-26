use yew::prelude::*;

#[cfg(feature = "client")]
use crate::content;
#[cfg(feature = "client")]
use crate::utils::external::ExternalResultContainer;
#[cfg(feature = "client")]
use blog_generic::entities::{ChatAnswer, ChatQuestion};
#[cfg(feature = "client")]
use gloo_net::http::Request;
#[cfg(feature = "client")]
use wasm_bindgen_futures::spawn_local;

use web_sys::HtmlTextAreaElement;

use crate::components::simple_title_card::SimpleTitleCard;

#[derive(Clone, PartialEq)]
struct ChatMessage {
    from_user: bool,
    text: String,
}

impl ChatMessage {
    fn ai(text: impl Into<String>) -> Self {
        Self {
            from_user: false,
            text: text.into(),
        }
    }

    fn user(text: impl Into<String>) -> Self {
        Self {
            from_user: true,
            text: text.into(),
        }
    }
}

const GREETING: &str = "Привет! Что хотите почитать?";

#[function_component(AiChat)]
pub fn ai_chat() -> Html {
    let messages = use_state(|| vec![ChatMessage::ai(GREETING)]);

    let question = use_state(String::new);
    let sending = use_state(|| false);

    let oninput = {
        let question = question.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            question.set(input.value());
        })
    };

    let send = {
        let messages = messages.clone();
        let question = question.clone();
        let sending = sending.clone();
        Callback::from(move |_| {
            if *sending {
                return;
            }
            let q = (*question).trim().to_string();
            if q.is_empty() {
                return;
            }

            messages.set({
                let mut msgs = (*messages).clone();
                msgs.push(ChatMessage::user(&q));
                msgs
            });
            question.set(String::new());
            sending.set(true);

            #[cfg(feature = "client")]
            {
                let messages = messages.clone();
                let sending = sending.clone();
                spawn_local(async move {
                    let url = format!("{}/chat", crate::API_URL);
                    let body = serde_json::to_string(&ChatQuestion { question: q }).unwrap();
                    let resp = Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(body)
                        .unwrap()
                        .send()
                        .await;

                    if let Ok(resp) = resp {
                        if let Ok(api) = resp.json::<content::API<ChatAnswer>>().await {
                            if let Ok(answer) = api.result() {
                                messages.set({
                                    let mut msgs = (*messages).clone();
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
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                send.emit(());
            }
        })
    };

    html! {
        <>
            <SimpleTitleCard>{ "AI рекомендации" }</SimpleTitleCard>
            { for (*messages).iter().map(|m| html! {
                <div class="card mb-3">
                    <div class="card-header d-flex align-items-center">
                        <i class={classes!("bi", if m.from_user { "bi-person-fill" } else { "bi-robot" }, "me-2")}></i>
                        { if m.from_user { "Вы" } else { "AI" } }
                    </div>
                    <div class="card-body">
                        <p class="card-text">{ &m.text }</p>
                    </div>
                </div>
            }) }
            <div class="mb-3">
                <textarea
                    class="form-control"
                    rows="3"
                    placeholder="Спросите, что почитать..."
                    value={(*question).clone()}
                    {oninput}
                    {onkeydown}
                ></textarea>
            </div>
            <div class="mb-3 d-grid gap-2">
                <button class="btn btn-light" {onclick} disabled={*sending}>{ "Отправить" }</button>
            </div>
        </>
    }
}
