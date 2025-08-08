use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::post_recommendation::*;
use crate::Route;

#[derive(Clone, serde::Deserialize)]
struct AccordionItem {
    title: String,
    body: String,
}

#[function_component(InformationMenu)]
pub fn information_menu() -> Html {
    let accordion_items = use_memo((), |_| {
        serde_json::from_str::<Vec<AccordionItem>>(crate::ACCORDION_JSON)
            .expect("wrong ACCORDION_JSON format")
    });
    let route = use_route::<Route>();
    let recommendation = if let Some(Route::Post { id, .. }) = route {
        html! { <div class="mt-3"><PostRecommendation id={ id } /></div> }
    } else {
        html! { <></> }
    };

    html! {
        <>
            <div class="accordion" id="accordionMain">
                {
                    accordion_items.iter().enumerate().map(|(index, item)| {
                        let item = item.clone();
                        let id = format!("collapse{index}");
                        let target = format!("#collapse{index}");
                        let expanded;
                        let mut button_classes = vec!["accordion-button"];
                        let mut content_classes = vec!["accordion-collapse", "collapse"];
                        if index == 0 {
                            expanded = "true";
                            content_classes.push("show");
                        } else {
                            expanded = "false";
                            button_classes.push("collapsed");
                        };
                        html! {
                            <div class="accordion-item">
                                <h2 class="accordion-header">
                                    <button
                                        class={ button_classes }
                                        type="button"
                                        data-bs-toggle="collapse"
                                        data-bs-target={ target }
                                        aria-expanded={ expanded }
                                        aria-controls={ id.clone() }
                                    >
                                        { item.title }
                                    </button>
                                </h2>
                                <div { id } class={ content_classes } data-bs-parent="#accordionMain">
                                    <div class="accordion-body">
                                        { Html::from_html_unchecked(AttrValue::from(item.body)) }
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            { recommendation }
        </>
    }
}
