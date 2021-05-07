// use reqwest::Client;
// use std::env;
// use std::string::String;

// fn get_env_var(key: &str) -> String {
//     match env::var(key) {
//         Ok(val) => val.to_string(),
//         Err(error) => error.to_string(),
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let token = get_env_var("TOKEN");

//     let response = Client::new()
//         .get("https://api.spacetraders.io/users/strewthmate")
//         .bearer_auth(token)
//         .send()
//         .await?
//         .text()
//         .await?;

//     println!("{:?}", response);

//     Ok(())
// }

use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
