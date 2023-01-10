use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use reqwasm::http::Request;
use gloo_console::log;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Delivery {
    id: u32,
    origin_address: String,
    delivery_address: String,
    preferred_pickup: String,
    expected_pickup: String,
    preferred_delivery: String,
    expected_delivery: String,
    status: String,
}

impl Delivery {
    fn display_delivery(&self) -> Html {
        html!{
            <div>
                <p> {"id: "} {&self.origin_address} </p>
                <p> {"origin_address: "} {&self.origin_address} </p>
                <p> {"delivery_address: "} {&self.delivery_address} </p>
                <p> {"preferred_pickup: "} {&self.preferred_pickup} </p>
                <p> {"expected_pickup: "} {&self.expected_pickup} </p>
                <p> {"preferred_delivery: "} {&self.preferred_delivery} </p>
                <p> {"expected_delivery: "} {&self.expected_delivery} </p>
                <p> {"status: "} {&self.status} </p>
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
                match res {
                    Ok(dels) => {
                        log!("deliveries: {}", to_string(&dels).unwrap());
                        deliveries.set(Ok(dels));
                    },
                    Err(e) => {
                        log!("error: {}", e);
                    }
                }
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