#![recursion_limit="256"]
#![feature(rustc_private)]

mod components;
mod intent;
mod search_regex;
mod structs;

use anyhow::Error;
use serde_json::json;
use std::string::ToString;
use yew::format::{Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;
use yew::prelude::*;
use wasm_bindgen::prelude::*;


use intent::get_response_from_intent;
use search_regex::get_response_from_regex;
use structs::*;

use self::{
    components::{
        chatbot_input::ChatbotInput,
        message::view_message,
    },
};

#[wasm_bindgen(module = "/chatbot_configuration.js")]
extern "C" {
    type ChatbotConfiguration;

    #[wasm_bindgen(constructor)]
    fn new() -> ChatbotConfiguration;

    #[wasm_bindgen(method, getter)]
    fn title(this: &ChatbotConfiguration) -> String;
    #[wasm_bindgen(method, setter)]
    fn set_title(this: &ChatbotConfiguration, title: String) -> ChatbotConfiguration;
    #[wasm_bindgen(method, getter)]
    fn typing_text(this: &ChatbotConfiguration) -> String;
    #[wasm_bindgen(method, setter)]
    fn set_typing_text(this: &ChatbotConfiguration, typing_text: String) -> ChatbotConfiguration;
    #[wasm_bindgen(method, getter)]
    fn use_nlu(this: &ChatbotConfiguration) -> bool;
    #[wasm_bindgen(method, setter)]
    fn set_use_nlu(this: &ChatbotConfiguration, use_nlu: bool) -> ChatbotConfiguration;
    #[wasm_bindgen(method, getter)]
    fn nlu_url(this: &ChatbotConfiguration) -> String;
    #[wasm_bindgen(method, setter)]
    fn set_nlu_url(this: &ChatbotConfiguration, nlu_url: String) -> ChatbotConfiguration;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub enum Msg {
    BeforeFetchData(String),
    FetchData,
    FetchNlu(Result<DataFromNlu, Error>),
    FetchRegex,
    AfterFetchBot,
    Ignore
}

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService,
    link: ComponentLink<Model>,
    title: String,
    typing: bool,
    typing_text: String,
    last_message: String,
    messages: Vec<Message>,
    nlu_url: String,
    use_nlu: bool,
    ft: Option<FetchTask>,
}

impl Model {
    fn view_data(&self) -> Html {
        let messages = &self.messages;
        if messages.len() > 0 {
            html! {
                <div>
                    { messages.iter().map(|message| view_message(&message.content, &message.r#type, &message.created_by)).collect::<Html>() }
                </div>
            }
        } else {
            html! {
                <div></div>
            }
        }
    }

    fn view_typing(&self) -> Html {
        if self.typing {
            html! {
                <div><p>{ &self.typing_text }</p></div>
            }
        }else{
            html! {
                <div></div>
            }
        }
    }

    fn fetch_nlu(&mut self, message: String) -> yew::services::fetch::FetchTask {
        let message = &json!({"content": message.to_string()});
        let callback = self.link.callback(
            move |response: Response<Json<Result<DataFromNlu, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchNlu(data)
                } else {
                    Msg::Ignore
                }
            },
        );
        let request = Request::builder()
            .method("POST")
            .header("Content-Type", "application/json")
            .uri(self.nlu_url.to_string())
            .body(Json(message))
            .unwrap();
        self.fetch_service.fetch_binary(request, callback).unwrap()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let chatbot_configuration = ChatbotConfiguration::new();

        Self {
            fetch_service: FetchService::new(),
            console: ConsoleService::new(),
            link,
            title: chatbot_configuration.title(),
            typing: false,
            typing_text: chatbot_configuration.typing_text(),
            messages: Vec::new(),
            nlu_url: chatbot_configuration.nlu_url(),
            last_message: "".to_string(),
            use_nlu: chatbot_configuration.use_nlu(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::BeforeFetchData(val) => {
                self.last_message=val.clone();
                self.update(Msg::FetchData);
            }
            Msg::FetchData => {
                self.typing=true;
                self.messages.push(Message{content: structs::ContentMessage{message:self.last_message.to_string(), choices: None}, created_by:"user".to_string(), r#type:"simple_message".to_string()});
                if self.use_nlu {
                    let nlu_task = self.fetch_nlu(self.last_message.to_string());
                    self.ft = Some(nlu_task);
                }else{
                    self.update(Msg::FetchRegex);
                }
            }
            Msg::FetchNlu(response) => {
                self.typing = false;
                if response.is_ok() {
                    let data = response.map(|child| child).ok();
                    let intent_name = match data {
                        None => "".to_string(),
                        Some(ref child) => child.intent.intentName.to_string(),
                    };
                    let slots = match data {
                        None => Vec::new(),
                        Some(ref child) => child.slots.iter().map(|grandchild| grandchild.rawValue.as_str()).collect(),
                    };
                    if intent_name != ""{
                        self.messages.push(get_response_from_intent(intent_name, slots, "bot".to_string()));
                    }else{
                        self.update(Msg::FetchRegex);
                    }
                } else {
                    self.update(Msg::FetchRegex);
                }
                self.update(Msg::AfterFetchBot);
            }
            Msg::FetchRegex => {
                self.typing = false;
                self.messages.push(Message{content: ContentMessage{message:get_response_from_regex(self.last_message.to_string()), choices: None}, created_by:"bot".to_string(), r#type:"simple_message".to_string()});
                self.update(Msg::AfterFetchBot);
            }
            Msg::AfterFetchBot => {
                return false;
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="chatbot_title">
                    { &self.title }
                </div>
                <div class="chatbot_messages">
                    { self.view_data() }
                    { self.view_typing() }
                </div>
                <div class="chatbot_input">
                    <ChatbotInput: onsignal=self.link.callback(Msg::BeforeFetchData) />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    App::<Model>::new().mount_to_body();
}
