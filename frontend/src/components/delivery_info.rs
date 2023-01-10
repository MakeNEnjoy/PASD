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
                <a href="/"> {"Home"} </a> <br />
                {"id: "} {&self.id} <br />
                {"origin_address: "} {&self.origin_address.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"delivery_address: "} {&self.delivery_address.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"preferred_pickup: "} {&self.preferred_pickup.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"expected_pickup: "} {&self.expected_pickup.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"preferred_delivery: "} {&self.preferred_delivery.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"expected_delivery: "} {&self.expected_delivery.clone().unwrap_or_else(|| "null".to_string())} <br />
                {"status: "} {&self.status.clone().unwrap_or_else(|| "null".to_string())} <br />
                <a href={format!("/update-date/{}", &self.id)}> {"Update Preferred Delivery Date and Time"} </a>
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
    log!("status: {}", response.status());
    log!("res: {}", &body);
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
    let deliveries= use_state(|| Err("Fetching deliveries".to_string()));
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