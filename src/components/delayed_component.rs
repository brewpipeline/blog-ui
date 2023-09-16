use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct DelayedComponentProps<D: PartialEq + Clone + 'static> {
    pub deps: D,
    pub component: Callback<D, Html>,
}

#[function_component(DelayedComponent)]
pub fn delayed_component<D: PartialEq + Clone + 'static>(props: &DelayedComponentProps<D>) -> Html {
    let DelayedComponentProps { deps, component } = props.clone();

    let component_state = use_state_eq(|| html! {});

    {
        let component_state = component_state.clone();
        use_effect_with(deps, move |deps| {
            component_state.set(component.emit(deps.clone()));
        })
    }

    (*component_state).clone()
}
