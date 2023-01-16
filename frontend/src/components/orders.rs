use yew::prelude::*;
use reqwasm::http::Request;
use gloo_console::log;
use serde_json::{to_string, from_str};
use super::webshop_model;

async fn get_orders() -> Result<webshop_model::Orders, String> {
    log!("fetching orders");
    let response = Request::get("/webshop/api/order/")
        .header("accept", "application/json")
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
        204 => Ok(webshop_model::Orders { orders: vec![] }),
        _ => Err("An error occurred".to_string())
    }
}

#[function_component(OrdersList)]
fn orders_page() -> Html {
    let orders = use_state(|| Err("Fetching deliveries".to_string()));
    {
        let orders = orders.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = get_orders().await;
                orders.set(res);
            });
        }, ());
    }
    match &*orders {
        Ok(webshop_model::Orders{orders}) => html! {
            <div>
                <ul>
                    { for orders.iter().map(|d| d.display()) }
                </ul>
            </div>
        },
        Err(e) => html! {
            <div>
                <p> {e} </p>
            </div>
        }
    }
}

pub fn orders_page() -> Html {
    html!{
        <>
            <a href="/logistics"> {"Home"} </a> <br />
            <h1> {"Web shop orders"} </h1>
            <OrdersList />
        </>
    }
}