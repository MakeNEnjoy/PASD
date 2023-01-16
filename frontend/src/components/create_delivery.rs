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
    StatusInput
};
use crate::router::Route;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Delivery {
    pub origin_address: Option<String>,
    pub delivery_address: Option<String>,
    pub preferred_pickup: Option<String>,
    pub expected_pickup: Option<String>,
    pub preferred_delivery: Option<String>,
    pub expected_delivery: Option<String>,
    pub webshop_id: Option<u32>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeliveryID {
    pub id: u32
}

impl Delivery {
    pub async fn post_request(&self) -> Result<DeliveryID, String> {
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
    UpdateWebShopID(String),
}

impl Component for Delivery {
    type Properties = ();
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Delivery::default()
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
            Msg::UpdateWebShopID(value) => self.webshop_id = Some(value.parse().unwrap()),
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
                log!("delivery: {:?}", to_string(&delivery).unwrap());
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
                <label> {"Expected Pickup"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedPickup) } /> <br />
                <label> {"Preferred Delivery"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdatePreferredDelivery) } /> <br />
                <label> {"Expected Delivery"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedDelivery) } /> <br />
                <label> {"Status"} </label>
                <StatusInput on_change={ctx.link().callback(Msg::UpdateStatus) } /> <br />
                <label> {"Webshop ID"} </label>
                <TextInput on_change={ctx.link().callback(Msg::UpdateWebShopID) } /> <br />
                <button type="submit" onclick={ctx.link().callback(|_| Msg::CreateDelivery) }> {"Create Delivery"} </button>
            </form>
        }
    }
}

pub fn create_delivery_page() -> Html {
    html! {
        <div>
            <h1> {"Create Delivery"} </h1>
            <a href="/"> {"Home"} </a>
            <Delivery />
        </div>
    }
}