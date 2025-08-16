use gloo_net::http::Request;
use web_sys::{FormData, HtmlInputElement};
use yew::prelude::*;

use crate::components::meta::*;
use crate::components::warning::*;
use crate::utils::{LoggedUserContext, LoggedUserState};

#[function_component(UploadImage)]
pub fn upload_image() -> Html {
    let file_input = use_node_ref();
    let message = use_state(|| None as Option<(String, u16)>);
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let meta = html! { <Meta title="Загрузка изображений" noindex=true /> };

    let not_auth_content = html! {
        <>
            { meta.clone() }
            <Warning text="Нужна авторизация для загрузки изображений!" />
        </>
    };

    if logged_user_context.is_not_inited() {
        return not_auth_content;
    }

    let LoggedUserState::ActiveAndLoaded { token, author } = logged_user_context.state().clone()
    else {
        return not_auth_content;
    };

    if author.editor != 1 {
        return html! {
            <>
                { meta.clone() }
                <Warning text="Загрузка изображений доступна только редакторам!" />
            </>
        };
    }

    let on_upload = {
        let file_input = file_input.clone();
        let message = message.clone();
        let token = token.clone();
        Callback::from(move |_| {
            let file_input = file_input.clone();
            let message = message.clone();
            let token = token.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let input = match file_input.cast::<HtmlInputElement>() {
                    Some(i) => i,
                    None => return,
                };
                let files = match input.files() {
                    Some(f) => f,
                    None => return,
                };
                let file = match files.get(0) {
                    Some(f) => f,
                    None => return,
                };
                let form = match FormData::new() {
                    Ok(f) => f,
                    Err(_) => return,
                };
                form.append_with_blob_and_filename("file", &file, &file.name())
                    .ok();
                let req = match Request::post("/upload/image")
                    .header("Token", &token)
                    .body(form)
                {
                    Ok(r) => r,
                    Err(err) => {
                        message.set(Some((err.to_string(), 0)));
                        return;
                    }
                };
                match req.send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        message.set(Some((text, status)));
                    }
                    Err(err) => {
                        message.set(Some((err.to_string(), 0)));
                    }
                }
            });
        })
    };

    let prompt = if let Some((text, status)) = (*message).clone() {
        let class = if (200..300).contains(&status) {
            "text-success"
        } else {
            "text-danger"
        };
        html! { <p class={class}>{ text }</p> }
    } else {
        html! {}
    };

    html! {
        <>
            { meta }
            <div class="d-flex flex-column gap-3">
                <input type="file" ref={file_input} />
                <button class="btn btn-light" onclick={on_upload}>{ "Загрузить" }</button>
                { prompt }
            </div>
        </>
    }
}
