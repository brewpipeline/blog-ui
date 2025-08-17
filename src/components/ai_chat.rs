use yew::prelude::*;

#[cfg(feature = "client")]
use gloo_net::http::Request;
#[cfg(feature = "client")]
use wasm_bindgen_futures::spawn_local;

#[cfg(feature = "client")]
use crate::content;
use crate::utils::*;
#[cfg(feature = "client")]
use blog_generic::entities::{ChatAnswer, ChatQuestion};

#[function_component(AiChat)]
pub fn ai_chat() -> Html {
    let messages = use_state(|| Vec::<(bool, String)>::new());
    let question = use_state(|| String::new());
    let sending = use_state(|| false);
    let expanded = use_state(|| false);

    let oninput = {
        let question = question.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
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
                msgs.push((true, q.clone()));
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
                                    msgs.push((false, answer.answer));
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

    if !*expanded {
        html! {
            <div class="ai-chat mt-3 w-100">
                <input
                    class="form-control"
                    placeholder="Ask what to read"
                    onclick={open_chat}
                    readonly=true
                />
            </div>
        }
    } else {
        html! {
            <div class="ai-chat card mt-3 w-100">
                <div class="card-header d-flex justify-content-between align-items-center">
                    { "AI рекомендации" }
                    <button type="button" class="btn-close" aria-label="Close" onclick={close_chat}></button>
                </div>
                <div class="chat-body card-body">
                    {
                        for (*messages).iter().map(|(is_user, msg)| {
                            let class = if *is_user {"chat-message user"} else {"chat-message ai"};
                            html!{ <div class={class}>{ msg }</div> }
                        })
                    }
                </div>
                <div class="card-footer d-flex">
                    <input
                        class="form-control me-2"
                        value={(*question).clone()}
                        {oninput}
                        {onkeydown}
                    />
                    <button class="btn btn-purple" {onclick} disabled={*sending}>{ "Отправить" }</button>
                </div>
            </div>
        }
    }
}
