use yew::prelude::*;
use yew::suspense::*;

use crate::utils::get::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LoadProps<I, P>
where
    I: PartialEq + Clone + 'static,
    P: PartialEq + Clone + 'static,
{
    pub params: P,
    pub component: Callback<Result<I, String>, Html>,
}

#[function_component(Load)]
pub fn load<I, P>(props: &LoadProps<I, P>) -> HtmlResult
where
    I: RequestableItem<P> + PartialEq + Clone + 'static,
    P: PartialEq + Clone + 'static,
{
    let LoadProps { params, component } = props.clone();

    let result = use_future_with_deps(
        move |params| async move { I::get((*params).clone()).await },
        params,
    )?;

    let component_result = match *result {
        Ok(ref item) => Ok(item.clone()),
        Err(ref err) => Err(err.to_string()),
    };

    Ok(component.emit(component_result))
}
