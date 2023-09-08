use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct SimpleTitleCardProps {
    pub children: Children,
}

#[function_component(SimpleTitleCard)]
pub fn simple_title_card(props: &SimpleTitleCardProps) -> Html {
    let SimpleTitleCardProps { children } = props.clone();
    html! {
        <div class="card mb-3">
            <div class="card-body">
                <h5 class="card-title mb-0 placeholder-glow">
                    { children }
                </h5>
            </div>
        </div>
    }
}
