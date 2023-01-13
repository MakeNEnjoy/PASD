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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Offer {
    pub price_in_cents: Option<u32>,
    pub expected_delivery_datetime: Option<String>,
    pub order_id: Option<u32>,
}

#[derive(Clone)]
struct OfferMaker {
    expected_pickup: Option<String>,
    offer: Offer,
    pickup_address: String,
    delivery_address: String,
    delivery: Option<WebShopDelivery>,
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
            _ => Err("An error occurred".to_string())
        }
    }
}

impl OfferMaker {
    fn handle_new_delivery(&mut self, delivery: WebShopDelivery, ctx: &Context<Self>) {
        log!("current delivery info: {}", to_string(&self.delivery).unwrap());
        log!("handling delivery: {}", to_string(&delivery).unwrap());
        let expected_delivery = self.offer.expected_delivery_datetime.clone();
        self.delivery = Some(delivery.clone());
        if delivery.status == Some("EXP".to_string()) {
            let offer_maker = self.clone();
            let navigator = ctx.link().navigator().unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                Delivery {
                    origin_address: Some(offer_maker.pickup_address),
                    delivery_address: Some(offer_maker.delivery_address),
                    preferred_pickup: None,
                    preferred_delivery: None,
                    expected_pickup: offer_maker.expected_pickup,
                    expected_delivery: expected_delivery,
                    status: Some("awaiting pickup".to_string()),
                }.post_request().await.map(|DeliveryID {id  }| {
                        navigator.push(&Route::Deliveries);
                    }).unwrap_or_else(|e| {
                        log!("error: {}", e);
                    });

            });

        }
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
    UpdateDelivery(WebShopDelivery),
}

impl Component for OfferMaker {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();
        let addresses = location.query::<AddressQuery>().unwrap();
        OfferMaker {
            offer: Offer {
                order_id: Some(ctx.props().order_id),
                
                ..Default::default()
            },
            pickup_address: addresses.pickup_address,
            delivery_address: addresses.delivery_address,
            expected_pickup: None,
            delivery: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdatePrice(price) => self.offer.price_in_cents = Some(price),
            Msg::UpdateExpectedDelivery(date) => self.offer.expected_delivery_datetime = Some(date),
            Msg::UpdateExpectedPickup(date) => self.expected_pickup = Some(date),
            Msg::UpdateDelivery(delivery) => self.handle_new_delivery(delivery, ctx),
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
                    offer.post_make_bid().await.map(|delivery| {
                        update_delivery.emit(delivery);
                    }).unwrap_or_else(|e| {
                        log!("error: {}", e);
                    });
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
                if let Some(delivery) = &self.delivery {
                    if delivery.status == Some("REJ".to_string()) {
                        <p> { "Your offer was rejected" } </p>
                    }
                
                }
            </>
        }
    }
}



pub fn create_webshop_delivery_page(id: u32) -> Html {
    html! {
        <>
            <h1> { "Make an offer" } </h1>
            <OfferMaker order_id={id} />
        </>
    }
}