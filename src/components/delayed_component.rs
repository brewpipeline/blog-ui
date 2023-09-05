use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct DelayedComponentProps {
    pub component: Callback<(), Html>,
}

#[function_component(DelayedComponent)]
pub fn delayed_component(props: &DelayedComponentProps) -> Html {
    let DelayedComponentProps {
        component: component_callback,
    } = props.clone();

    let component = use_state_eq(|| html! {});

    #[cfg(feature = "client")]
    {
        let component = component.clone();
        use_effect_with((), move |_| {
            component.set(component_callback.emit(()));
        })
    }

    (*component).clone()
}
