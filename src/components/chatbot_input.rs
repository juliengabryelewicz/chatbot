use yew::prelude::*;
use yew::events::IKeyboardEvent;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, KeyPressEvent, ShouldRender};

pub struct ChatbotInput {
    link: ComponentLink<Self>,
    input_value: String,
    onsignal: Callback<String>,
}

pub enum Msg {
    UpdateSearchText(String),
    Submit,
    Ignore,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub onsignal: Callback<String>,
}

impl Component for ChatbotInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ChatbotInput {
            link,
            input_value: "".to_string(),
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateSearchText(val) => {
                self.input_value = val;
            }
            Msg::Submit => {
                let message = self.input_value.clone();
                self.input_value="".to_string();
                self.onsignal.emit(message.to_string());
            }
            Msg::Ignore => {
                return false;
            }
        }
        false
    }

    // This is for props

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="text",
                placeholder="Enter your message",
                autocomplete="off",
                value=&self.input_value,
                oninput=self.link.callback(|e: InputData| { Msg::UpdateSearchText(e.value) }),
                onkeypress=self.link.callback(|e: KeyPressEvent| {
                    if e.key() == "Enter" { Msg::Submit } else { Msg::Ignore }
                }),
            />
        }
    }
}
