#[cfg(feature = "client")]
use blog_generic::entities::*;
#[cfg(feature = "client")]
use gloo::utils::window;
#[cfg(feature = "client")]
use web_sys::{ScrollBehavior, ScrollToOptions};
use yew::prelude::*;

#[cfg(feature = "client")]
use wasm_bindgen_futures::spawn_local;
#[cfg(feature = "client")]
use web_sys::HtmlTextAreaElement;

use crate::components::meta::*;
#[cfg(feature = "client")]
use crate::content::*;
#[cfg(feature = "client")]
use crate::utils::*;

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

#[function_component(ChatGPT)]
pub fn chatgpt() -> Html {
    let messages = use_state(|| {
        vec![ChatMessage::ai(
            "Привет! Я — ChatGPT, адаптированный под этот блог. О чём бы ты хотел почитать?",
        )]
    });

    let question = use_state(String::new);
    let sending = use_state(|| false);

    #[cfg(feature = "client")]
    let oninput = {
        let question = question.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            question.set(input.value());
        })
    };
    #[cfg(not(feature = "client"))]
    let oninput = Callback::from(|_| {});

    #[cfg(feature = "client")]
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

            let mut msgs = (*messages).clone();
            msgs.push(ChatMessage::user(q.clone()));
            messages.set(msgs.clone());
            question.set(String::new());
            sending.set(true);

            let messages = messages.clone();
            let sending = sending.clone();
            spawn_local(async move {
                match API::<ChatAnswer>::get(ChatQuestion { question: q }).await {
                    Ok(api_resp) => match api_resp {
                        API::Success {
                            identifier: _,
                            description: _,
                            data,
                        } => {
                            msgs.push(ChatMessage::ai(data.answer));
                            messages.set(msgs);
                        }
                        API::Failure {
                            identifier: _,
                            reason,
                        } => {
                            let reason_text =
                                reason.unwrap_or_else(|| "неизвестная причина".to_string());
                            msgs.push(ChatMessage::ai(format!(
                                "Произошла ошибка при получении ответа: {}",
                                reason_text
                            )));
                            messages.set(msgs);
                        }
                    },
                    Err(_) => {}
                }
                sending.set(false);
            });
        })
    };

    #[cfg(feature = "client")]
    let onclick = {
        let send = send.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            send.emit(());
        })
    };
    #[cfg(not(feature = "client"))]
    let onclick = Callback::from(|_| {});

    #[cfg(feature = "client")]
    let onkeydown = {
        let send = send.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                send.emit(());
            }
        })
    };
    #[cfg(not(feature = "client"))]
    let onkeydown = Callback::from(|_| {});

    #[cfg(feature = "client")]
    {
        let messages = messages.clone();
        use_effect_with(messages, move |_| {
            let scroll_to_options = ScrollToOptions::new();
            scroll_to_options.set_left(0.0);
            scroll_to_options.set_top(9999999.0);
            scroll_to_options.set_behavior(ScrollBehavior::Instant);
            window().scroll_to_with_scroll_to_options(&scroll_to_options);
        });
    }

    html! {
        <>
            <Meta title="ChatGPT" noindex=true />
            { for (*messages).iter().map(|m| html! {
                <div class="card mb-3">
                    <div class="card-header d-flex align-items-center">
                        <i class={classes!("bi", if m.from_user { "bi-person-fill" } else { "bi-openai" }, "me-2")}></i>
                        { if m.from_user { "Вы" } else { "ChatGPT" } }
                    </div>
                    <div class="card-body">
                        <p class="card-text">{ &m.text }</p>
                    </div>
                </div>
            }) }
            /*<div class="input-group mb-3">
                <input
                    class="form-control"
                    rows="3"
                    placeholder="Спросите что-нибудь…"
                    value={(*question).clone()}
                    {oninput}
                    {onkeydown}
                />
                <button class="btn btn-light" {onclick} disabled={*sending}>{ "Отправить" }</button>
            </div>*/
            <div class="mb-3">
                <textarea
                    class="form-control"
                    rows="3"
                    placeholder="Спросите что-нибудь…"
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
