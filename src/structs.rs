extern crate serde_derive;

use serde_derive::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ValueNlu {
    pub kind: String,
    pub value: String
}

#[derive(Deserialize, Debug)]
pub struct RangeNlu {
    pub start: i32,
    pub end: i32
}

#[derive(Deserialize, Debug)]
pub struct IntentNlu {
    pub intentName: String,
    pub confidenceScore: f32
}

#[derive(Deserialize, Debug)]
pub struct SlotNlu {
    pub rawValue: String,
    pub r#value: ValueNlu,
    pub alternatives: Vec<String>,
    pub range: RangeNlu,
    pub entity: String,
    pub slotName: String
}

#[derive(Deserialize, Debug)]
pub struct DataFromNlu {
    pub input: String,
    pub intent: IntentNlu,
    pub slots: Vec<SlotNlu>,
    pub alternatives: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct ContentMessage {
    pub message: String,
    pub choices: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub content: ContentMessage,
    pub created_by: String,
    pub r#type: String,
}
