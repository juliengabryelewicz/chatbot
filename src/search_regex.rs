extern crate regex;

use regex::Regex;

pub fn get_response_from_regex(intent: String) -> String {

	let input_regex = Regex::new(
        r#"(?x)
        (hello) |
        (evening)
        "#
    ).unwrap();

	let captures = input_regex.captures(&intent).map(|captures| {
        captures.iter().skip(1).flat_map(|c| c).map(|c| c.as_str()).collect::<Vec<_>>()
    });

	match captures.as_ref().map(|c| c.as_slice()) {
		Some(["hello"]) => "hello to you".to_string(),
		Some(["evening"]) => "good evening to you".to_string(),
		_ => "Sorry, I do not understand what you say".to_string(),
	}

}
