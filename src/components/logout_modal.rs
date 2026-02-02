use yew::prelude::*;

use crate::lang;
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
            if !logged_user_context.is_not_inited() {
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">
                                { lang::AUTH_LOGOUT_TITLE }
                            </h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            if logged_user_context.token() != None {
                                <p class="mb-3">
                                    { lang::AUTH_LOGOUT_CONFIRM }
                                </p>
                                <div class="d-grid gap-2">
                                    <button
                                        type="button"
                                        class="btn btn-info"
                                        data-bs-dismiss="modal"
                                        onclick={ move |_| logged_user_context.dispatch(LoggedUserState::LoggedOut) }
                                    >
                                        { lang::AUTH_LOGOUT }
                                    </button>
                                </div>
                            } else {
                                <h5 class="mb-2 mt-2 text-center"> { lang::AUTH_NOT_AUTHORIZED } </h5>
                            }
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
