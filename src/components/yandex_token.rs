#[cfg(feature = "client")]
use gloo::utils::document;
#[cfg(feature = "client")]
use gloo::utils::window;
#[cfg(feature = "client")]
use web_sys::Element;
use yew::prelude::*;

use crate::components::delayed_component::*;

#[function_component(YandexToken)]
pub fn yandex_token() -> Html {
    html! {
        <>
            <script id="yandexTokenScript" src="https://yastatic.net/s3/passport-sdk/autofill/v1/sdk-suggest-token-with-polyfills-latest.js"></script>
            <DelayedComponent<()> component={ |_| {
                #[cfg(feature = "client")]
                {
                    let script: Element = document().create_element("script").unwrap();
                    script.set_attribute("type", "text/javascript").unwrap();
                    script.set_inner_html(format!("
                        function yaSendSuggestTokenAction() {{
                            window.YaSendSuggestToken('{origin}', {{}})
                        }}
                        if (typeof window.YaSendSuggestToken === 'undefined') {{
                            document.getElementById('yandexTokenScript').onload = yaSendSuggestTokenAction
                        }} else {{
                            yaSendSuggestTokenAction()
                        }}
                    ", origin = window().origin()).as_str());
                    Html::VRef(script.into())
                }
                #[cfg(not(feature = "client"))]
                unreachable!()
            }} deps={ () } />
        </>
    }
}
