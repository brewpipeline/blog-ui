use noneifempty::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::author_image::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct AuthorCardProps {
    pub author: Option<Author>,
    pub link_to: bool,
}

#[function_component(AuthorCard)]
pub fn author_card(props: &AuthorCardProps) -> Html {
    let AuthorCardProps { author, link_to } = props.clone();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let in_progress = use_state_eq(|| false);
    let is_blocked = use_state_eq(|| false);

    {
        let is_blocked = is_blocked.clone();
        use_effect_with(author.clone(), move |author| {
            is_blocked.set(author.as_ref().map(|a| a.blocked == 1).unwrap_or(false));
        })
    }

    #[cfg(feature = "client")]
    {
        let author = author.clone();
        let logged_user_context = logged_user_context.clone();
        let in_progress = in_progress.clone();
        let is_blocked = is_blocked.clone();
        use_effect_with(in_progress, move |in_progress| {
            if logged_user_context.is_not_inited() || !**in_progress {
                return;
            }

            let (Some(author), Some(token)) = (author, (*logged_user_context).token().cloned())
            else {
                return;
            };

            let in_progress = in_progress.clone();
            let is_blocked = is_blocked.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let block_author_request = API::<()>::get(Tokened {
                    token,
                    params: BlockAuthorIdParams {
                        id: author.id,
                        block: !*is_blocked,
                    },
                });
                match block_author_request.await {
                    Ok(block_author_result) => match block_author_result {
                        API::Success {
                            identifier: _,
                            description: _,
                            data: _,
                        } => {
                            is_blocked.set(!*is_blocked);
                        }
                        API::Failure {
                            identifier: _,
                            reason: _,
                        } => {}
                    },
                    Err(_) => {}
                }
                in_progress.set(false);
            });
        });
    }

    let onclick = {
        let in_progress = in_progress.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if *in_progress {
                return;
            }
            in_progress.set(true);
        })
    };

    let main_content = html! {
        <div class="row g-0">
            <div class="col-4">
                <div class="img-block rounded-start" style="height:220px;width:100%;overflow:hidden;">
                    <AuthorImage author={ author.clone() } />
                    if !link_to && !logged_user_context.is_not_inited() && author != None {
                        if (*logged_user_context)
                            .author()
                            .map(|a| a.editor == 1)
                            .unwrap_or(false)
                        {
                            <button
                                type="button"
                                style="position:absolute;left:5px;bottom:5px;"
                                class={ classes!("btn", if *is_blocked { "btn-success" } else { "btn-danger" }) }
                                disabled={ *in_progress }
                                { onclick }
                            >
                                <i class="bi bi-hammer"></i>
                                {
                                    if !(*is_blocked) { " Бан " } else { " Разбан " }
                                }
                            </button>
                        }
                    }
                </div>
            </div>
            <div class="col">
                <div class="card-body">
                    <h4 class="card-title placeholder-glow">
                        if let Some(author) = author.as_ref() {
                            {
                                if author.blocked == 1 {
                                    "(Имя скрыто)".to_owned()
                                } else {
                                    format!(
                                        "{} {}",
                                        author.first_name
                                            .clone()
                                            .none_if_empty()
                                            .unwrap_or("(Имя не указано)".to_owned()),
                                        author.last_name
                                            .clone()
                                            .unwrap_or_default()
                                    )
                                }
                            }
                            if author.editor == 1 {
                                { " " }
                                <i style="color:#6ea5ff;font-size:1rem;"> { "Главный редактор" } </i>
                            }
                            if *is_blocked {
                                { " " }
                                <i style="color:#ff5252;font-size:1rem;"> { "Заблокирован" } </i>
                            }
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span> { " " }
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </h4>
                    <p class="card-text placeholder-glow">
                        <small class="text-body-secondary">
                            if let Some(author) = author.as_ref() {
                                { author_slug_formatter(&author) }
                            } else {
                                <span class="placeholder col-2 bg-secondary"></span>
                            }
                        </small>
                    </p>
                    <p class="card-text placeholder-glow">
                        if let Some(author) = author.as_ref() {
                            {
                                if author.blocked == 1 {
                                    "(Email скрыт)".to_owned()
                                } else {
                                    author.email
                                        .clone()
                                        .none_if_empty()
                                        .unwrap_or("(Email не указан)".to_owned())
                                }
                            }
                        } else {
                            <span class="placeholder col-4 bg-secondary"></span>
                        }
                    </p>
                    <p class="card-text placeholder-glow">
                        if let Some(author) = author.as_ref() {
                            {
                                if author.blocked == 1 {
                                    "(Информация о себе скрыта)".to_owned()
                                } else {
                                    author.status
                                        .clone()
                                        .none_if_empty()
                                        .unwrap_or("(Информация о себе отсутствует)".to_owned())
                                }
                            }
                        } else {
                            <span class="placeholder col-4 bg-secondary"></span> { " " }
                            <span class="placeholder col-2 bg-secondary"></span> { " " }
                            <span class="placeholder col-3 bg-secondary"></span> { " " }
                            <span class="placeholder col-2 bg-secondary"></span> { " " }
                            <span class="placeholder col-5 bg-secondary"></span> { " " }
                            <span class="placeholder col-2 bg-secondary"></span> { " " }
                            <span class="placeholder col-3 bg-secondary"></span> { " " }
                        }
                    </p>
                </div>
            </div>
        </div>
    };

    html! {
        <div class="card mb-3">
            if let (Some(author), true) = (author.as_ref(), link_to) {
                <Link<Route, (), Author>
                    classes="text-decoration-none"
                    to={ Route::Author { slug: author.slug.clone() } }
                    state={ Some(author.clone()) }
                >
                    { main_content }
                </Link<Route, (), Author>>
            } else {
                { main_content }
            }
        </div>
    }
}
