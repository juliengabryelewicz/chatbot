pub fn get_response_from_intent(intent: String, slots: Vec<&str>) -> String {

	let s = String::from(intent);

	match s.as_str() {
        "prepareBeverage" => ["I'm preparing your ", slots[0], " right now"].concat().to_string(),
        _ => "Sorry, I do not understand your request".to_string(),
    }
}
