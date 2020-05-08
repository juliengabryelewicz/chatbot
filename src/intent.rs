pub fn get_response_from_intent(intent: String) -> String {  

	let s = String::from(intent);

	match s.as_str() {
        "prepareBeverage" => "I'm preparing it now".to_string(),
        _ => "Sorry, I do not understand your request".to_string(),
    }
}