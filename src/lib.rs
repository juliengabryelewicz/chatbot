#![recursion_limit="256"]
#![feature(rustc_private)]

mod intent;
mod components;

use anyhow::Error;
use serde_derive::{Deserialize};
use serde_json::json;
use std::string::ToString;
use yew::format::{Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;
use intent::get_response_from_intent;

use self::{
    components::{
        chatbot_input::ChatbotInput,
        message::view_message,
    },
};

pub enum Msg {
    FetchData(String),
    FetchResponse(Result<DataFromNlu, Error>),
    Ignore
}

#[derive(Deserialize, Debug)]
pub struct ValueNlu {
    kind: String,
    value: String
}

#[derive(Deserialize, Debug)]
pub struct RangeNlu {
    start: i32,
    end: i32
}

#[derive(Deserialize, Debug)]
pub struct IntentNlu {
    intentName: String,
    confidenceScore: f32
}

#[derive(Deserialize, Debug)]
pub struct SlotNlu {
    rawValue: String,
    r#value: ValueNlu,
    alternatives: Vec<String>,
    range: RangeNlu,
    entity: String,
    slotName: String
}

#[derive(Deserialize, Debug)]
pub struct DataFromNlu {
    input: String,
    intent: IntentNlu,
    slots: Vec<SlotNlu>,
    alternatives: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Message {
    content: String,
    created_by: String,
    r#type: String,
}

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService,
    link: ComponentLink<Model>,
    title: String,
    typing: bool,
    messages: Vec<Message>,
    ft: Option<FetchTask>,
}

impl Model {
    fn view_data(&self) -> Html { 
        let messages = &self.messages;   
        if messages.len() > 0 {
            html! {
                <div>
                    { messages.iter().map(|message| view_message(&message.content, &message.r#type)).collect::<Html>() }
                </div>
            }
        } else {
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
                    Msg::FetchResponse(data)
                } else {
                    Msg::Ignore
                }
            },
        );
        let request = Request::builder()
            .method("POST")
            .header("Content-Type", "application/json")
            .uri("http://localhost:8080/parse".to_string())
            .body(Json(message))
            .unwrap();
        self.fetch_service.fetch_binary(request, callback).unwrap()
    }

}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_service: FetchService::new(),
            console: ConsoleService::new(),
            link,
            title: "Chat Title".to_string(),
            typing: false,
            messages: Vec::new(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData(val) => {
                self.messages.push(Message{content: val.to_string(), r#type: "User".to_string(), created_by: "User".to_string()});
                let nlu_task = self.fetch_nlu(val);
                self.ft = Some(nlu_task);
            }
            Msg::FetchResponse(response) => {
                self.typing = false;
                if response.is_ok() {
                    let data = response.map(|data| data.intent.intentName).ok();
                    let intent_name = match data {
                        None => "".to_string(),
                        Some(ref intentName) => intentName.to_string(),
                    };
                    self.messages.push(Message{content: get_response_from_intent(intent_name), r#type: "Bot".to_string(), created_by: "Bot".to_string()});
                } else {
                    self.messages.push(Message{content: get_response_from_intent("".to_string()), r#type: "Bot".to_string(), created_by: "Bot".to_string()});
                    self.console.log("ko");
                }
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
                </div>
                <div class="chatbot_input">
                    <ChatbotInput: onsignal=self.link.callback(Msg::FetchData) />
                </div>
            </div>
        }
    }
}