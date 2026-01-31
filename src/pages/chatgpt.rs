#[cfg(feature = "client")]
use blog_generic::entities::*;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(feature = "client")]
use wasm_bindgen_futures::spawn_local;
#[cfg(feature = "client")]
use web_sys::HtmlTextAreaElement;

use crate::Route;
use crate::components::meta::*;
#[cfg(feature = "client")]
use crate::content::*;
use crate::lang;
use crate::utils::*;

fn render_text_with_links(text: &str) -> Html {
    let mut nodes: Vec<Html> = Vec::new();
    let mut buf = String::new();

    let mut i = 0;
    let len = text.len();
    while i < len {
        let rest = &text[i..];
        let is_http = rest.starts_with("http://") || rest.starts_with("https://");
        let is_path = rest.starts_with('/');

        if is_http || is_path {
            if !buf.is_empty() {
                nodes.push(html! { { buf.clone() } });
                buf.clear();
            }

            let mut j = i;
            while j < len {
                let ch = text[j..].chars().next().unwrap();
                if ch.is_whitespace() {
                    break;
                }
                j += ch.len_utf8();
            }

            let mut link = text[i..j].to_string();
            let mut trailing = String::new();
            while let Some(last) = link.chars().last() {
                if matches!(last, ',' | '.' | '!' | '?' | ':' | ';' | ')') {
                    link.pop();
                    trailing.insert(0, last);
                } else {
                    break;
                }
            }

            let maybe_route = if link.starts_with('/') {
                Route::recognize_path(&link)
            } else if let Some(scheme_pos) = link.find("://") {
                let after_scheme = &link[scheme_pos + 3..];
                if let Some(slash_pos) = after_scheme.find('/') {
                    let path = &after_scheme[slash_pos..];
                    Route::recognize_path(path)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(route) = maybe_route {
                let label = link.clone();
                nodes.push(html! { <Link<Route> to={route}>{ label }</Link<Route>> });
            } else {
                let href = link.clone();
                nodes.push(html! { <a href={href.clone()} target="_blank" rel="noopener noreferrer">{ href }</a> });
            }

            if !trailing.is_empty() {
                buf.push_str(&trailing);
            }
            i = j;
        } else {
            let ch = rest.chars().next().unwrap();
            if ch == '\n' {
                if !buf.is_empty() {
                    nodes.push(html! { { buf.clone() } });
                    buf.clear();
                }
                nodes.push(html! { <br/> });
                i += 1;
                continue;
            }
            if ch == '\r' {
                i += 1;
                continue;
            }
            buf.push(ch);
            i += ch.len_utf8();
        }
    }

    if !buf.is_empty() {
        nodes.push(html! { { buf } });
    }

    html! { for node in nodes { {node} } }
}

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
    let messages = use_state(|| vec![ChatMessage::ai(lang::CHATGPT_GREETING)]);

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
                                reason.unwrap_or_else(|| lang::CHATGPT_UNKNOWN_REASON.to_string());
                            msgs.push(ChatMessage::system(lang::chatgpt_error(&reason_text)));
                            messages.set(msgs);
                        }
                    },
                    Err(_) => {
                        msgs.push(ChatMessage::system(lang::CHATGPT_NETWORK_ERROR.to_string()));
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
                                ChatFrom::User => lang::CHATGPT_USER,
                                ChatFrom::ChatGpt => "ChatGPT",
                                ChatFrom::System => lang::CHATGPT_SYSTEM,
                            }
                        }
                    </div>
                    <div class="card-body">
                        <p class="card-text">
                            {
                                match m.from {
                                    ChatFrom::ChatGpt => render_text_with_links(&m.text),
                                    _ => render_text_with_newlines(&m.text),
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
                                <span>{ lang::CHATGPT_TYPING }</span>
                            </div>
                        </div>
                    </div>
                }
            } else { html!{} } }
            <div class="mb-3">
                <textarea
                    class="form-control"
                    rows="3"
                    placeholder={ lang::CHATGPT_PLACEHOLDER }
                    value={(*question).clone()}
                    {oninput}
                    {onkeydown}
                />
            </div>
            <div class="mb-3 d-grid gap-2">
                <button class="btn btn-light" {onclick} disabled={*sending}>{ lang::COMMON_SEND }</button>
            </div>
        </>
    }
}
