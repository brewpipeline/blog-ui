#[cfg(feature = "client")]
use blog_generic::entities::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::AttrValue;

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
enum ChatFrom {
    User,
    ChatGpt,
    System,
}

#[derive(Clone, PartialEq)]
struct ChatMessage {
    from: ChatFrom,
    text: String,
}

impl ChatMessage {
    fn ai(text: impl Into<String>) -> Self {
        Self {
            from: ChatFrom::ChatGpt,
            text: text.into(),
        }
    }

    fn user(text: impl Into<String>) -> Self {
        Self {
            from: ChatFrom::User,
            text: text.into(),
        }
    }

    fn system(text: impl Into<String>) -> Self {
        Self {
            from: ChatFrom::System,
            text: text.into(),
        }
    }
}

#[function_component(ChatGPT)]
pub fn chatgpt() -> Html {
    let messages = use_state(|| {
        vec![ChatMessage::ai(
            "Привет! Я — ChatGPT, адаптированный под этот блог. Я в курсе свежих публикаций и помогу подобрать интересное. О чём бы ты хотел почитать?",
        )]
    });

    let question = use_state(String::new);
    let sending = use_state(|| false);
    let session_id = use_state(Uuid::new_v4);

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
        let session_id = session_id.clone();
        Callback::from(move |_| {
            if *sending {
                return;
            }
            let final_question = (*question).trim().to_string();
            if final_question.is_empty() {
                return;
            }

            let mut msgs = (*messages).clone();
            msgs.push(ChatMessage::user(final_question.clone()));
            messages.set(msgs.clone());
            question.set(String::new());
            sending.set(true);

            let messages = messages.clone();
            let sending = sending.clone();
            let session_id = *session_id;
            spawn_local(async move {
                match API::<ChatAnswer>::get(ChatGptQuestion {
                    session_id,
                    question: ChatQuestion {
                        question: final_question,
                    },
                })
                .await
                {
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
                            msgs.push(ChatMessage::system(format!(
                                "Произошла ошибка при получении ответа: {}",
                                reason_text
                            )));
                            messages.set(msgs);
                        }
                    },
                    Err(_) => {
                        msgs.push(ChatMessage::system(
                            "Произошла ошибка сети при получении ответа".to_string(),
                        ));
                        messages.set(msgs);
                    }
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
            scroll_to_bottom_instant();
        });
    }

    html! {
        <>
            <Meta title="ChatGPT" noindex=true />
            { for (*messages).iter().map(|m| html! {
                <div class="card mb-3">
                    <div class="card-header d-flex align-items-center">
                        <i class={classes!(
                            "bi",
                            match m.from {
                                ChatFrom::User => "bi-person-fill",
                                ChatFrom::ChatGpt => "bi-openai",
                                ChatFrom::System => "bi-exclamation-triangle",
                            },
                            "me-2"
                        )}></i>
                        {
                            match m.from {
                                ChatFrom::User => "Вы",
                                ChatFrom::ChatGpt => "ChatGPT",
                                ChatFrom::System => "Система",
                            }
                        }
                    </div>
                    <div class="card-body">
                        <p class="card-text">
                            {
                                match m.from {
                                    ChatFrom::ChatGpt => Html::from_html_unchecked(AttrValue::from(m.text.clone())),
                                    _ => html! { &m.text },
                                }
                            }
                        </p>
                    </div>
                </div>
            }) }
            { if *sending {
                html! {
                    <div class="card mb-3">
                        <div class="card-header d-flex align-items-center">
                            <i class={classes!("bi","bi-openai","me-2")}></i>
                            { "ChatGPT" }
                        </div>
                        <div class="card-body">
                            <div class="d-flex align-items-center">
                                <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                <span>{ "Печатает…" }</span>
                            </div>
                        </div>
                    </div>
                }
            } else { html!{} } }
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
