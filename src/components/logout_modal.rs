use yew::prelude::*;

use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct LogoutModalProps {
    pub id: &'static str,
}

#[function_component(LogoutModal)]
pub fn logout_modal(props: &LogoutModalProps) -> Html {
    let LogoutModalProps { id } = props.clone();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    html! {
        <div class="modal fade" { id } tabindex="-1">
            <div class="modal-dialog">
                <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">
                        { "Выход" }
                    </h5>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <p>
                        { "Вы точно хотите выйти?" }
                    </p>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                        { "Закрыть" }
                    </button>
                    <button
                        type="button"
                        class="btn btn-primary"
                        data-bs-dismiss="modal"
                        onclick={ move |_| logged_user_context.dispatch(LoggedUserState::None) }
                    >
                        { "Выйти" }
                    </button>
                </div>
                </div>
            </div>
        </div>
    }
}
