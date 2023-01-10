use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use reqwasm::http::Request;
use gloo_console::log;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Delivery {
    id: u32,
    origin_address: Option<String>,
    delivery_address: Option<String>,
    preferred_pickup: Option<String>,
    expected_pickup: Option<String>,
    preferred_delivery: Option<String>,
    expected_delivery: Option<String>,
    status: Option<String>,
}

impl Delivery {
    fn display_delivery(&self) -> Html {
        html!{
            <div>
                {"id: "} {&self.id} <br />
                {"origin_address: "} {&self.origin_address.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"delivery_address: "} {&self.delivery_address.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"preferred_pickup: "} {&self.preferred_pickup.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"expected_pickup: "} {&self.expected_pickup.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"preferred_delivery: "} {&self.preferred_delivery.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"expected_delivery: "} {&self.expected_delivery.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"status: "} {&self.status.clone().unwrap_or_else(|| "null".to_string())} <br />
                <br />
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
    // log!("status: {}", response.status());
    // log!("res: {}", &body);
    match response.status() {
        200 => from_str(&body).map_err(|e| e.to_string()),
        204 => Ok(vec![]),
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
                let res = get_deliveries().await;
                deliveries.set(res);
            });
        }, ());
    }
    match &*deliveries {
        Ok(dels) => html! {
            <div>
                <h1> {"Deliveries"} </h1>
                <ul>
                    { for dels.iter().map(|d| d.display_delivery()) }
                </ul>
            </div>
        },
        Err(e) => html! {
            <div>
                <h1> {"Error when getting deliveries"} </h1>
                <p> {e} </p>
            </div>
        }
    }
}