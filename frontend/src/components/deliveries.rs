use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use reqwasm::http::Request;
use gloo_console::log;

use super::create_webshop_delivery::WebShopDelivery;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Delivery {
    id: u32,
    origin_address: Option<String>,
    delivery_address: Option<String>,
    preferred_pickup: Option<String>,
    expected_pickup: Option<String>,
    preferred_delivery: Option<String>,
    expected_delivery: Option<String>, 
    webshop_id: Option<u32>,
    webshop_delivery: Option<WebShopDelivery>,
    status: Option<String>,
}

impl Delivery {
    fn display_delivery(&self) -> Html {
        html!{
            <div>
                // {"id: "} {&self.id} <br />
                {"origin_address: "} {&self.origin_address.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"delivery_address: "} {&self.delivery_address.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"preferred_pickup: "} {&self.preferred_pickup.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"expected_pickup: "} {&self.expected_pickup.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"preferred_delivery: "} {&self.preferred_delivery.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"expected_delivery: "} {&self.expected_delivery.clone().unwrap_or_else(|| "null".to_string())} <br />
                // {"webshop_id: "} {&self.webshop_id.clone().map_or("null".to_string(), |id| id.to_string())} <br />
                {"status: "} {&self.status.clone().unwrap_or_else(|| "null".to_string())} <br />
                if let Some(webshop_id) = &self.webshop_id {
                    <a href={format!("/update-status/{}?webshop_id={}", &self.id, webshop_id)}> {"Update Status"} </a> <br />
                    if let Some(webshop_delivery) = &self.webshop_delivery {
                        if Some("EXP".to_string()) == webshop_delivery.status {
                            <a href={format!("/create-label/{}", webshop_id)}> {"Label needs to be uploaded!"} </a> <br />
                        } else {
                            // {"webshop_delivery: "} {webshop_delivery.status.clone().unwrap()}
                        }
                    } else {
                        { "Can't find delivery in webshop!" }
                    }
                } else {
                    <a href={format!("/update-status/{}", &self.id)}> {"Update Status"} </a> <br />
                }
                <br /> <br />
            </div>
        }
    }
}

async fn get_deliveries() -> Result<Vec<Delivery>, String> {
    let response = Request::get("/api/deliveries")
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
        204 => Err("There are no deliveries".to_string()),
        _ => Err("An error occurred".to_string())
    }
}

#[function_component(Deliveries)]
pub fn deliveries_page() -> Html {
    let deliveries = use_state(|| Err("Fetching deliveries".to_string()));
    {
        let deliveries = deliveries.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let mut res = get_deliveries().await;
                if let Ok(ref mut dels) = res {
                    let dels_filter = dels.iter_mut().filter(|d| d.status != Some("delivered".to_string()));
                    for del in dels_filter {
                        if let Some(webshop_id) = del.webshop_id {
                            del.webshop_delivery = WebShopDelivery::get_delivery(webshop_id).await.ok();
                        }
                    }
                }
                deliveries.set(res);
            });
        }, ());
    }
    match &*deliveries {
        Ok(dels) => html! {
            <div>
                <a href="/logistics"> {"Home"} </a> <br />
                <h1> {"Deliveries"} </h1>
                <ul>
                    { for dels.iter().map(|d| {
                        if d.status != Some("delivered".to_string()) {
                            d.display_delivery()
                        } else {
                            html! {}
                        }
                    }) }
                </ul>
            </div>
        },
        Err(e) => html! {
            <div>
                <a href="/logistics"> {"Home"} </a> <br />
                <h1> {"Deliveries"} </h1>
                <p> {e} </p>
            </div>
        }
    }
}