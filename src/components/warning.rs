use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct WarningProps {
    pub text: String,
}

#[function_component(Warning)]
pub fn warning(props: &WarningProps) -> Html {
    html! {
        <div class="alert alert-primary d-flex align-items-center" role="alert">
            { props.text.clone() }
        </div>
    }
}