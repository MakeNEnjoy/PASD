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

async fn get_delivery(id: u32) -> Result<Delivery, String> {
    let response = Request::get(&format!("/api/deliveries/{}", id))
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
        404 => Err("Delivery not found".to_string()),
        _ => Err("An error occurred".to_string())
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[function_component(DeliveryInfo)]
pub fn deliveriy_info_page(props: &Props) -> Html {
    let id = props.id;
    let deliveries = use_state(|| Err("Fetching deliveries".to_string()));
    {
        let deliveries = deliveries.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = get_delivery(id).await;
                deliveries.set(res);
            });
        }, ());
    }
    match &*deliveries {
        Ok(del) => html! {
            <div>
                <h1> {"Delivery"} </h1>
                {del.display_delivery()}
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