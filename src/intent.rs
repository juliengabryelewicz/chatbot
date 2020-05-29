use crate::structs;

pub fn get_response_from_intent(intent: String, slots: Vec<&str>,created_by:String) -> structs::Message {

	let s = String::from(intent);

	match s.as_str() {
        "prepareBeverage" => structs::Message{content: structs::ContentMessage{message:["I'm preparing your ", slots[0], " right now"].concat().to_string(), choices: None}, created_by:"bot".to_string(), r#type:"simple_message".to_string()},
        _ => structs::Message{content: structs::ContentMessage{message:"Sorry, I do not understand your request".to_string(), choices: None}, created_by:"bot".to_string(), r#type:"simple_message".to_string()},
    }
}
