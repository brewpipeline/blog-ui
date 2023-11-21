use yew::prelude::*;

#[function_component(PersonAddImg)]
pub fn person_add_img() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-add" viewBox="0 0 16 16">
            <path d="M12.5 16a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7Zm.5-5v1h1a.5.5 0 0 1 0 1h-1v1a.5.5 0 0 1-1 0v-1h-1a.5.5 0 0 1 0-1h1v-1a.5.5 0 0 1 1 0Zm-2-6a3 3 0 1 1-6 0 3 3 0 0 1 6 0ZM8 7a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"/>
            <path d="M8.256 14a4.474 4.474 0 0 1-.229-1.004H3c.001-.246.154-.986.832-1.664C4.484 10.68 5.711 10 8 10c.26 0 .507.009.74.025.226-.341.496-.65.804-.918C9.077 9.038 8.564 9 8 9c-5 0-6 3-6 4s1 1 1 1h5.256Z"/>
        </svg>
    }
}

#[function_component(CardHeadingImg)]
pub fn card_heading_img() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-card-heading" viewBox="0 0 16 16">
            <path d="M14.5 3a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5h13zm-13-1A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h13a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2h-13z"/>
            <path d="M3 8.5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h6a.5.5 0 0 1 0 1h-6a.5.5 0 0 1-.5-.5zm0-5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5v-1z"/>
        </svg>
    }
}

#[function_component(SearchImg)]
pub fn search_img() -> Html {
    html! {
        <svg aria-label="Поиск" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-search" viewBox="0 0 16 16">
            <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"></path>
        </svg>
    }
}

#[function_component(FilePostImg)]
pub fn file_post_img() -> Html {
    html! {
        <svg style="width: 1em; height: 1em;" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-file-post" viewBox="0 0 16 16">
            <path d="M4 3.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 .5.5v8a.5.5 0 0 1-.5.5h-7a.5.5 0 0 1-.5-.5v-8z"></path>
            <path d="M2 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2zm10-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1z"></path>
        </svg>
    }
}

#[function_component(PencilSquareImg)]
pub fn pencil_square_img() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pencil-square" viewBox="0 0 16 16" style="margin-bottom: 2px;">
            <path d="M15.502 1.94a.5.5 0 0 1 0 .706L14.459 3.69l-2-2L13.502.646a.5.5 0 0 1 .707 0l1.293 1.293zm-1.75 2.456-2-2L4.939 9.21a.5.5 0 0 0-.121.196l-.805 2.414a.25.25 0 0 0 .316.316l2.414-.805a.5.5 0 0 0 .196-.12l6.813-6.814z"/>
            <path fill-rule="evenodd" d="M1 13.5A1.5 1.5 0 0 0 2.5 15h11a1.5 1.5 0 0 0 1.5-1.5v-6a.5.5 0 0 0-1 0v6a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 .5-.5H9a.5.5 0 0 0 0-1H2.5A1.5 1.5 0 0 0 1 2.5v11z"/>
        </svg>
    }
}

#[function_component(EyeSlashFillImg)]
pub fn eye_slash_fill_img() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-eye-slash-fill" viewBox="0 0 16 16" style="margin-bottom: 2px;">
            <path d="m10.79 12.912-1.614-1.615a3.5 3.5 0 0 1-4.474-4.474l-2.06-2.06C.938 6.278 0 8 0 8s3 5.5 8 5.5a7.029 7.029 0 0 0 2.79-.588zM5.21 3.088A7.028 7.028 0 0 1 8 2.5c5 0 8 5.5 8 5.5s-.939 1.721-2.641 3.238l-2.062-2.062a3.5 3.5 0 0 0-4.474-4.474L5.21 3.089z"/>
            <path d="M5.525 7.646a2.5 2.5 0 0 0 2.829 2.829l-2.83-2.829zm4.95.708-2.829-2.83a2.5 2.5 0 0 1 2.829 2.829zm3.171 6-12-12 .708-.708 12 12-.708.708z"/>
        </svg>
    }
}

#[function_component(TrashImg)]
pub fn trash_img() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16" style="margin-bottom: 2px;">
            <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5Zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5Zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6Z"/>
            <path d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1ZM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118ZM2.5 3h11V2h-11v1Z"/>
        </svg>
    }
}

#[function_component(HammerImg)]
pub fn hammer_img() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-hammer" viewBox="0 0 16 16" style="margin-bottom: 2px;">
            <path d="M9.972 2.508a.5.5 0 0 0-.16-.556l-.178-.129a5.009 5.009 0 0 0-2.076-.783C6.215.862 4.504 1.229 2.84 3.133H1.786a.5.5 0 0 0-.354.147L.146 4.567a.5.5 0 0 0 0 .706l2.571 2.579a.5.5 0 0 0 .708 0l1.286-1.29a.5.5 0 0 0 .146-.353V5.57l8.387 8.873A.5.5 0 0 0 14 14.5l1.5-1.5a.5.5 0 0 0 .017-.689l-9.129-8.63c.747-.456 1.772-.839 3.112-.839a.5.5 0 0 0 .472-.334z"/>
        </svg>
    }
}

#[cfg(feature = "yandex")]
#[function_component(YandexImg)]
pub fn yandex_img() -> Html {
    html! {
        <svg viewBox="0 0 377 90.9" xmlns="http://www.w3.org/2000/svg" width="2500" height="603" style="width: 100%; height: 100%; padding: 50px;">
            <path d="M218.2 90.9c7.2 0 13.1-3.4 17.3-9.7l.8 8.4h14.2V0h-15.3v32.4c-3.8-5.8-9.6-8.9-16.3-8.9-14.8 0-25.2 12.5-25.2 34.2 0 21.3 10.1 33.2 24.5 33.2zm94.4-5.6V72.8c-4.8 3.2-12.8 6.1-20.3 6.1-11.2 0-15.5-5.3-16.1-16.1h37v-8.2c0-22.6-9.9-31.1-25.3-31.1-18.7 0-27.6 14.3-27.6 33.9 0 22.6 11.1 33.5 30.7 33.5 9.8 0 17-2.5 21.6-5.6zM147.3 43.1c2.9-3.5 7.3-6.4 12.9-6.4 5.4 0 7.8 2.3 7.8 7.2v45.8h15.3V42.3c0-12.9-5.1-18.6-17.7-18.6-9.1 0-14.6 3.4-17.7 6.4h-.8l-.4-5.4h-15v64.9H147zm-27.9 2.3c0-15.7-8-21.7-24.3-21.7-10.1 0-18.2 4.3-22.8 7.6v13.3c4.9-4 12.4-8.6 21-8.6 7.3 0 10.7 2.6 10.7 9.6v4.1h-2.4c-23.5 0-33.9 7.6-33.9 21.2 0 12.5 8 19.7 19.9 19.7 9 0 12.9-3 15.9-6.1h.6c.1 1.7.6 3.8 1.1 5.1h15c-.5-5.3-.8-10.6-.8-15.9zm240.3 44.3H377l-21.2-33.4 18.3-31.4h-15.3l-11.1 19.6-12.4-19.6h-17.4L337.5 56l-20.4 33.8h15.6l13-21.9zM222.8 35.6c8.3 0 12.4 6.6 12.4 21.5 0 15.1-4.4 21.7-13 21.7-8.4 0-12.5-6.4-12.5-21.2-.1-15.3 4.3-22 13.1-22zm64.9 0c7.6 0 9.9 6.3 9.9 14.4v1.3h-21.4c.3-10.3 4.1-15.7 11.5-15.7zM104.1 74.2c-1.9 2.9-5.6 5.1-11 5.1-6.4 0-9.7-3.7-9.7-9.4 0-7.5 5.3-10.1 18.4-10.1h2.2v14.4z"/><path d="M45 74.4v15.2H29.4V64L0 0h16.3l22.9 50c4.4 9.6 5.8 12.9 5.8 24.4zM74.3 0L55.2 43.3H39.3L58.5 0z" fill="#fc3f1d"/>
        </svg>
    }
}

#[function_component(CounterclockwiseImg)]
pub fn counterclockwise_img() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-counterclockwise" viewBox="0 0 16 16" style="margin-bottom: 2px;">
            <path fill-rule="evenodd" d="M8 3a5 5 0 1 1-4.546 2.914.5.5 0 0 0-.908-.417A6 6 0 1 0 8 2v1z"/>
            <path d="M8 4.466V.534a.25.25 0 0 0-.41-.192L5.23 2.308a.25.25 0 0 0 0 .384l2.36 1.966A.25.25 0 0 0 8 4.466z"/>
        </svg>
    }
}
