use crate::structs;

use yew::{html, Html};


pub fn view_message(content: &structs::ContentMessage, r#type:&str, created_by:&str) -> Html {
    match r#type {
        "simple_message" => {
            html! {
                <div class=(String::from(created_by))>
                    <div>{ &content.message }</div>
                </div>
            }
        }
        _ => {
            html! {
                <div class=(String::from(created_by))>
                    <div>{ &content.message }</div>
                </div>
            }
        }
    }
}
