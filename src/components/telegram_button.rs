use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TelegramButtonProps {
    pub onauth: String,
}

#[function_component(TelegramButton)]
pub fn optional_image(props: &TelegramButtonProps) -> Html {
    let TelegramButtonProps { onauth } = props.clone();

    html! {
        <div style="height: 40px;">
            <script
                async=true
                src="https://telegram.org/js/telegram-widget.js?22"
                data-telegram-login={ crate::TELEGRAM_BOT_LOGIN }
                data-size="large"
                data-radius="5"
                data-onauth={ onauth }
                data-request-access="write"
            ></script>
        </div>
    }
}
