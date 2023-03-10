use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use serde_json::{to_string, from_str};
use reqwasm::http::Request;
use gloo_console::log;
use super::delete_delivery::DeleteDelivery;
use super::text_input::{
    TextInput,
    DateInput,
    StatusInput
};
use crate::router::Route;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Delivery {
    #[serde(skip_serializing)]
    id: Option<u32>,
    preferred_delivery: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DeliveryID {
    id: u32
}

impl Delivery {
    async fn post_request(&self) -> Result<DeliveryID, String> {
        log!("sending: {}", to_string(&self).unwrap());
        let response = Request::put(&format!("/api/deliveries/{}", self.id.unwrap()))
            .header("Content-Type", "application/json")
            .body(to_string(&self).unwrap())
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let body = response
            .text()
            .await
            .map_err(|e| e.to_string())?;
        log!("status: {}", response.status());
        log!("res: {}", &body);
        match response.status() {
            200 => from_str(&body).map_err(|e| e.to_string()),
            _ => Err("An error occurred".to_string())
        }
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u32,
}

pub enum Msg {
    UpdareDelivery,
    UpdatePreferredDelivery(String),
}

impl Component for Delivery {
    type Properties = Props;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            id: Some(props.id),
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdareDelivery => (),
            Msg::UpdatePreferredDelivery(value) => self.preferred_delivery = Some(value),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
        let onsubmit = {
            let navigator = navigator.clone();
            let delivery = self.clone();
            Callback::from(move |e: SubmitEvent| {
                let navigator = navigator.clone();
                let delivery = delivery.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let delivery = delivery.clone();
                    let navigator = navigator.clone();
                    e.prevent_default();
                    delivery.post_request().await.map(|DeliveryID{id}| {
                        let new_route = Route::Delivery{id};
                        navigator.push(&new_route);

                    }).unwrap_or_else(|e| {
                        log!("error: {}", e);
                    });
                });
            })
        };
        html!{
            <form {onsubmit}>
                <label> {"Preferred Delivery"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdatePreferredDelivery) } /> <br />
                <button type="submit" onclick={ctx.link().callback(|_| Msg::UpdareDelivery) }> {"Update Preerred Delivery Date"} </button>
                <DeleteDelivery id = {self.id.unwrap()} />
            </form>
        }
    }
}

pub fn update_preferred_delivery_page(id: u32) -> Html {
    html! {
        <div>
            <h1> {"Update Preferred Delivery Date"} </h1>
            <a href="/"> {"Home"} </a>
            <Delivery id = {id} />
        </div>
    }
}