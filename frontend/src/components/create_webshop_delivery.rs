use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use gloo_console::log;
use serde_json::{to_string, from_str};
use super::webshop_model::{Order, AddressQuery};
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use super::create_delivery::{Delivery, DeliveryID};
use super::text_input::{
    NumberInput,
    DateInput
};
use crate::router::Route;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebShopDelivery {
    pub expected_deliver_datetime: Option<String>,
    pub actual_deliver_datetime: Option<String>,
    pub order_id: u32,
    pub cost_in_cents: u32,
    pub status: Option<String>,
    pub id: u32
}
impl WebShopDelivery {
    pub async fn get_delivery(id: u32) -> Result<WebShopDelivery, String> {
        let response = Request::get(&format!("/webshop/api/delivery/{}", id))
            .header("x-api-key", "46XiHoFBHG7sViWGTx7a")
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Offer {
    pub price_in_cents: Option<u32>,
    pub expected_delivery_datetime: Option<String>,
    pub order_id: u32,
}

#[derive(Clone)]
struct OfferMaker {
    expected_pickup: Option<String>,
    offer: Offer,
    pickup_address: String,
    delivery_address: String,
    delivery: Result<WebShopDelivery, String>,
}

impl Offer {
    async fn post_make_bid(&self) -> Result<WebShopDelivery, String> {
        let mut offer = self.clone();
        if let Some(date) = &offer.expected_delivery_datetime {
            offer.expected_delivery_datetime = Some(format!("{}.000Z", date))
        }
        log!("sending: {}", to_string(&offer).unwrap());
        let response = Request::post("/webshop/api/delivery/")
            .header("Content-Type", "application/json")
            .header("x-api-key", "46XiHoFBHG7sViWGTx7a")
            .body(to_string(&offer).unwrap())
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
            _ => Err(body)
        }
    }
}

impl OfferMaker {
    fn handle_new_delivery(&mut self, delivery: Result<WebShopDelivery, String>, ctx: &Context<Self>) {
        log!("current delivery info: {}", to_string(&self.delivery).unwrap_or("None".to_string()));
        self.delivery = delivery.clone();
        if let Ok(delivery) = delivery {
            log!("handling delivery: {}", to_string(&delivery).unwrap());
            let expected_delivery = self.offer.expected_delivery_datetime.clone();
            if delivery.status == Some("EXP".to_string()) {
                let offer_maker = self.clone();
                let navigator = ctx.link().navigator().unwrap();
                wasm_bindgen_futures::spawn_local(async move {
                    Delivery {
                        origin_address: Some(offer_maker.pickup_address),
                        delivery_address: Some(offer_maker.delivery_address),
                        preferred_pickup: None,
                        preferred_delivery: None,
                        webshop_id: Some(delivery.id),
                        expected_pickup: offer_maker.expected_pickup,
                        expected_delivery: expected_delivery,
                        status: Some("awaiting pickup".to_string()),
                    }.post_request().await.map(|DeliveryID {id  }| {
                            // navigator.push(&Route::Deliveries);
                            log!("delivery successfully created: {}", id);
                        }).unwrap_or_else(|e| {
                            log!("error: {}", e);
                        });
                });
            }
        }
    }


    fn resend_delivery_to_backend(&self, ctx: &Context<Self>) {
        let id = self.offer.order_id;
        let callback = ctx.link().callback(Msg::UpdateDelivery);
        wasm_bindgen_futures::spawn_local(async move {
            let delivery = WebShopDelivery::get_delivery(id).await;
            callback.emit(delivery);
        });
    }
        
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub order_id: u32
}

pub enum Msg {
    UpdatePrice(u32),
    UpdateExpectedDelivery(String),
    UpdateExpectedPickup(String),
    UpdateDelivery(Result<WebShopDelivery, String>),
    ResendDelivery,
}

impl Component for OfferMaker {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();
        let addresses = location.query::<AddressQuery>().unwrap();
        OfferMaker {
            offer: Offer {
                order_id: ctx.props().order_id,
                
                ..Default::default()
            },
            pickup_address: addresses.pickup_address,
            delivery_address: addresses.delivery_address,
            expected_pickup: None,
            delivery: Err("No offer made yet".to_string())
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdatePrice(price) => self.offer.price_in_cents = Some(price),
            Msg::UpdateExpectedDelivery(date) => self.offer.expected_delivery_datetime = Some(date),
            Msg::UpdateExpectedPickup(date) => self.expected_pickup = Some(date),
            Msg::UpdateDelivery(delivery) => self.handle_new_delivery(delivery, ctx),
            Msg::ResendDelivery => self.resend_delivery_to_backend(ctx),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let navigator = ctx.link().navigator().unwrap();
        let onsubmit = {
            let navigator = navigator.clone();
            let update_delivery = ctx.link().callback(Msg::UpdateDelivery);
            let offer = self.offer.clone();
            Callback::from(move |e: SubmitEvent| {
                e.prevent_default();
                let navigator = navigator.clone();
                let offer = offer.clone();
                let update_delivery = update_delivery.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let offer = offer.clone();
                    let update_delivery = update_delivery.clone();
                    let navigator = navigator.clone();
                    update_delivery.emit(offer.post_make_bid().await);
                });
            })
        };
        
        let on_change_price = {
            let update_price = ctx.link().callback(Msg::UpdatePrice);
            Callback::from(move |price: String| {
                update_price.emit(price.parse().unwrap());
            })
        };


        html!{
            <>
                <p>{ "Pickup address: " } {&self.pickup_address}</p>
                <p>{ "Delivery address: "} {&self.delivery_address}</p>
                <form {onsubmit} >
                    <label>{ "Price in cents" }</label>
                    <NumberInput on_change={on_change_price} /><br />
                    <label>{ "Expected pickup date" }</label>
                    <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedPickup)} /><br />
                    <label>{ "Expected delivery date" }</label>
                    <DateInput on_change={ctx.link().callback(Msg::UpdateExpectedDelivery)} /><br />
                    <input type="submit" value="Submit" />
                </form>
                <h2> { "Offer status" } </h2>
                {
                    match &self.delivery {
                        Ok(delivery) if delivery.status == Some("EXP".to_string()) => html!{
                            <>
                                <p> { "Your offer was accepted" } </p>
                                <a href="/deliveries"> { "Go to current deliveries" } </a>
                            </>
                        },
                        Ok(delivery) if delivery.status == Some("REJ".to_string()) => html!{
                            <p> { "Your offer was rejected" } </p>
                        },
                        Err(e) if e == "{\"detail\":\"Order is already being delivered\"}" => html!{
                            <>
                                <p> { e } </p>
                                <p> { "Only click this button if you think the delivery is not in the Disruptive Delivery backend database." } </p> 
                                <button onclick={ctx.link().callback(|_| Msg::ResendDelivery)}> { "Update delivery in backend" } </button>
                            </>
                        },
                        Err(e) => html!{
                            <p> { e } </p>
                        },
                        _ => html!{
                            <p> { "Unexpected status" } </p>
                        }
                    }
                }
            </>
        }
    }
}



pub fn create_webshop_delivery_page(id: u32) -> Html {
    html! {
        <>
            <a href="/orders"> { "Back to orders" } </a>
            <h1> { "Make an offer" } </h1>
            <OfferMaker order_id={id} />
        </>
    }
}