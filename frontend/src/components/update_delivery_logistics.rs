use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use serde_json::{to_string, from_str};
use reqwasm::http::Request;
use gloo_console::log;
use super::create_webshop_delivery::WebShopDelivery;
use super::delete_delivery::DeleteDelivery;
use super::text_input::{
    DateInput,
    StatusInput
};
use crate::router::Route;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Delivery {
    #[serde(skip_serializing)]
    id: Option<u32>,
    expected_pickup: Option<String>,
    expected_delivery: Option<String>,
    status: Option<String>,
    webshop_id: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct DeliveryID {
    id: u32
}

#[derive(Serialize, Deserialize)]
struct WebShopIDQuery {
    webshop_id: Option<u32>
}

#[derive(Serialize, Deserialize, Clone)]
struct WebShopStatus {
    #[serde(skip_serializing)]
    delivery_id: u32,
    status: String,
    actual_deliver_datetime: Option<String>,
}

impl WebShopStatus {
    async fn patch_request(status: Self) -> Result<WebShopDelivery, String> {
        let response = Request::patch(&format!("/webshop/api/delivery/{}", status.delivery_id))
            .header("Content-Type", "application/json")
            .header("x-api-key", "46XiHoFBHG7sViWGTx7a")
            .body(to_string(&status).unwrap())
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

    fn picked_up_package(&self) {
        let status = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            Self::patch_request(status).await;
        });
    }
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
    UpdateDelivery,
    UpdateStatus(String),
    UpdateExpectedPickup(String),
    UpdateExpectedDelivery(String),
}

impl Component for Delivery {
    type Properties = Props;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();
        let id_query = location.query::<WebShopIDQuery>().unwrap();
        let props = ctx.props();
        Self {
            id: Some(props.id),
            webshop_id: id_query.webshop_id,
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateDelivery => (),
            Msg::UpdateStatus(value) => {
                self.status = Some(value.clone());
            },
            Msg::UpdateExpectedPickup(value) => self.expected_pickup = Some(value),
            Msg::UpdateExpectedDelivery(value) => self.expected_delivery = Some(value),
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
                if let Some(status) = &delivery.status {
                    match status.as_str() {
                        "in transit" => if let Some(webshop_id) = delivery.webshop_id {
                            let status = WebShopStatus {
                                delivery_id: webshop_id,
                                status: "TRN".to_string(),
                                actual_deliver_datetime: None,
                            };
                            status.picked_up_package();
                        },
                        "delivered" => if let Some(webshop_id) = delivery.webshop_id {
                            let status = WebShopStatus {
                                delivery_id: webshop_id,
                                status: "DEL".to_string(),
                                actual_deliver_datetime: delivery.expected_delivery.clone(),
                            };
                            status.picked_up_package();
                        },
                        _ => log!("other status")
                    }
                }
                wasm_bindgen_futures::spawn_local(async move {
                    let delivery = delivery.clone();
                    let navigator = navigator.clone();
                    e.prevent_default();
                    delivery.post_request().await.map(|DeliveryID{id}| {
                        let new_route = Route::Deliveries;
                        navigator.push(&new_route);

                    }).unwrap_or_else(|e| {
                        log!("error: {}", e);
                    });
                });
            })
        };
        html!{
            <form {onsubmit}>
                <label> {"Status"} </label>
                <StatusInput on_change={ctx.link().callback(Msg::UpdateStatus) } /> <br />
                <label> {"Expected Pickup"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedPickup) } /> <br />
                <label> {"Expected Delivery"} </label>
                <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedDelivery) } /> <br />
                <button type="submit" onclick={ctx.link().callback(|_| Msg::UpdateDelivery) }> {"Update Status"} </button>
                <DeleteDelivery id = {self.id.unwrap()} />
            </form>
        }
    }
}

pub fn update_delivery_logistics_page(id: u32) -> Html {
    html! {
        <div>
            <a href="/deliveries"> {"Deliveries"} </a>
            <h1> {"Update Status"} </h1>
            <Delivery id = {id} />
        </div>
    }
}