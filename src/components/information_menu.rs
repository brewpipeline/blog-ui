use yew::prelude::*;

#[function_component(InformationMenu)]
pub fn information_menu() -> Html {
    html! {
        <div class="accordion" id="accordionMain">
            {
                crate::ACCORDION_ITEMS.iter().enumerate().map(|(index, item)| {
                    let id = format!("collapse{index}");
                    let target = format!("#collapse{index}");
                    let expanded;
                    let mut button_classes = "accordion-button".to_string();
                    let mut content_classes = "accordion-collapse collapse".to_string();
                    if index == 0 {
                        expanded = "true";
                        content_classes += " show"
                     } else {
                        expanded = "false";
                        button_classes += " collapsed";
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
                                    { item.0 }
                                </button>
                            </h2>
                            <div { id } class={ content_classes } data-bs-parent="#accordionMain">
                                <div class="accordion-body">
                                    { Html::from_html_unchecked(AttrValue::from(item.1)) }
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
