use yew::{html, Html};

pub fn view_message(content: &str, r#type:&str, created_by:&str) -> Html {
    match r#type {
        "simple_message" => {
            html! {
                <div class=(String::from(r#type))>
                    <div>{ &content }</div>
                </div>
            }
        }
        _ => {
            html! {
                <div class=(String::from(r#type))>
                    <div>{ &content }</div>
                </div>
            }
        }
    }
}
