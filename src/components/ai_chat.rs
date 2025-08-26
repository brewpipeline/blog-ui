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
#[cfg(feature = "client")]
use gloo::storage::{LocalStorage, Storage};

#[function_component(AiChat)]
pub fn ai_chat() -> Html {
    #[cfg(feature = "client")]
    const STORAGE_KEY: &str = "ai-chat";

    // (is_user, is_pending, text)
    let chat = use_state(|| {
        #[cfg(feature = "client")]
        {
            let mut history: Vec<(bool, bool, String)> =
                LocalStorage::get(STORAGE_KEY).unwrap_or_default();
            if history.is_empty() {
                history.push((false, false, "Привет! Что хотите почитать?".to_string()));
            }
            history
        }
        #[cfg(not(feature = "client"))]
        {
            vec![(false, false, "Привет! Что хотите почитать?".to_string())]
        }
    });
    let sending = use_state(|| false);
    let expanded = use_state(|| false);

    #[cfg(feature = "client")]
    {
        let chat_value = (*chat).clone();
        use_effect_with(chat_value, move |chat_vec: &Vec<(bool, bool, String)>| {
            LocalStorage::set(STORAGE_KEY, chat_vec).ok();
            || ()
        });
    }

    let oninput = {
        let chat = chat.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            chat.set({
                let mut state = (*chat).clone();
                if let Some((is_user, is_pending, text)) = state.last_mut() {
                    if *is_user && *is_pending {
                        *text = input.value();
                    } else {
                        state.push((true, true, input.value()));
                    }
                } else {
                    state.push((true, true, input.value()));
                }
                state
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
            let q = (*chat)
                .last()
                .filter(|(is_user, is_pending, _)| *is_user && *is_pending)
                .map(|(_, _, s)| s.trim().to_string())
                .unwrap_or_default();
            if q.is_empty() {
                return;
            }

            chat.set({
                let mut state = (*chat).clone();
                if let Some((_, pending, _)) = state.last_mut() {
                    *pending = false;
                }
                state
            });
            sending.set(true);

            #[cfg(feature = "client")]
            {
                let chat = chat.clone();
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
                                chat.set({
                                    let mut state = (*chat).clone();
                                    if let Some((_, pending, _)) =
                                        state.iter_mut().rev().find(|(is_user, _, _)| *is_user)
                                    {
                                        *pending = false;
                                    }
                                    state.push((false, false, answer.answer));
                                    state
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
                        value={(*chat)
                            .iter()
                            .rev()
                            .find(|(is_user, _, _)| *is_user)
                            .map(|(_, _, q)| q.clone())
                            .unwrap_or_default()}
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
                    {
                        for (*chat)
                            .iter()
                            .filter(|(_, pending, _)| !pending)
                            .map(|(is_user, _, msg)| {
                                let alignment = if *is_user {"justify-content-end"} else {"justify-content-start"};
                                let class = if *is_user {"chat-message user"} else {"chat-message ai"};
                                let icon = if *is_user {"bi-person-circle"} else {"bi-robot"};
                                html! {
                                    <div class={classes!("d-flex", alignment)}>
                                        <div class={class}>
                                            <i class={classes!("bi", icon)}></i>
                                            <span>{ msg }</span>
                                        </div>
                                    </div>
                                }
                            })
                    }
                </div>
                <div class="card-footer d-flex">
                    <input
                        class="form-control me-2"
                        value={(*chat)
                            .last()
                            .filter(|(is_user, is_pending, _)| *is_user && *is_pending)
                            .map(|(_, _, q)| q.clone())
                            .unwrap_or_default()}
                        {oninput}
                        {onkeydown}
                    />
                    <button class="btn btn-purple" {onclick} disabled={*sending}>{ "Отправить" }</button>
                </div>
            </div>
        </div>
    }
}
