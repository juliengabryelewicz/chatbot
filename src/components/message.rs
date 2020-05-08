use yew::{html, Html};

pub fn view_message(content: &str, r#type:&str) -> Html {    
    html! {
        <div class=(String::from(r#type))>
            <div>{ &content }</div>
        </div>
    }
}