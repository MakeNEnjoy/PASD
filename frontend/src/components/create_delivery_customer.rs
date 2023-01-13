use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use serde_json::{to_string, from_str};
use reqwasm::http::Request;
use gloo_console::log;
use super::text_input::{
    TextInput,
    DateInput,
};
use crate::router::Route;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Delivery {
    origin_address: Option<String>,
    delivery_address: Option<String>,
    preferred_pickup: Option<String>,
    expected_pickup: Option<String>,
    preferred_delivery: Option<String>,
    expected_delivery: Option<String>,
    status: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DeliveryID {
    id: u32
}

impl Delivery {
    async fn post_request(&self) -> Result<DeliveryID, String> {
        log!("sending: {}", to_string(&self).unwrap());
        let response = Request::post("/api/deliveries")
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
            201 => from_str(&body).map_err(|e| e.to_string()),
            _ => Err("An error occurred".to_string())
        }
    }
}

pub enum Msg {
    CreateDelivery,
    UpdateOriginAddress(String),
    UpdateDeliveryAddress(String),
    UpdatePreferredPickup(String),
    UpdateExpectedPickup(String),
    UpdatePreferredDelivery(String),
    UpdateExpectedDelivery(String),
    UpdateStatus(String),
}

impl Component for Delivery {
    type Properties = ();
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Delivery {
            status: Some("awaiting pickup".to_string()),
            ..Delivery::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateDelivery => (),
            Msg::UpdateOriginAddress(value) => self.origin_address = Some(value),
            Msg::UpdateDeliveryAddress(value) => self.delivery_address = Some(value),
            Msg::UpdatePreferredPickup(value) => self.preferred_pickup = Some(value),
            Msg::UpdateExpectedPickup(value) => self.expected_pickup = Some(value),
            Msg::UpdatePreferredDelivery(value) => self.preferred_delivery = Some(value),
            Msg::UpdateExpectedDelivery(value) => self.expected_delivery = Some(value),
            Msg::UpdateStatus(value) => self.status = Some(value),
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
                    delivery.post_request().await.map(|DeliveryID {id  }| {
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
                <label> {"Origin Address"} </label>
                <TextInput on_change={ctx.link().callback(Msg::UpdateOriginAddress) } /> <br />
                <label> {"Delivery Address"} </label>
                <TextInput on_change={ctx.link().callback(Msg::UpdateDeliveryAddress) } /> <br />
                <label> {"Preferred Pickup"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdatePreferredPickup) } /> <br />
                // <label> {"Expected Pickup"} </label>
                // <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedPickup) } /> <br />
                <label> {"Preferred Delivery"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdatePreferredDelivery) } /> <br />
                // <label> {"Expected Delivery"} </label>
                // <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedDelivery) } /> <br />
                // <label> {"Status"} </label>
                // <StatusInput on_change={ctx.link().callback(Msg::UpdateStatus) } /> <br />
                <button type="submit" > {"Create Delivery"} </button>
            </form>
        }
    }
}

pub fn create_delivery_customer_page() -> Html {
    html! {
        <div>
            <h1> {"Create Delivery"} </h1>
            <a href="/"> {"Home"} </a>
            <p> { "If you want to deliver the package to disruptive delivery yourself, leave the origin address and preferred pickup time blank!" } </p>
            <Delivery />
        </div>
    }
}