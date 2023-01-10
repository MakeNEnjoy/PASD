use yew::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use reqwasm::http::Request;
use gloo_console::log;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Person {
    name: String,
    street_and_number: String,
    zipcode: String,
    city: String,
    country: String
}

impl Person {
    fn display(&self) -> Html {
        html!{
            <>
                {"name: "} {self.name.clone()} <br />
                {"street_and_number: "} {self.street_and_number.clone()} <br />
                {"zipcode: "} {self.zipcode.clone()} <br />
                {"city: "} {self.city.clone()} <br />
                {"country: "} {self.country.clone()} <br />
            </>
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Order {
    send_date: String,
    x_in_mm: u32,
    y_in_mm: u32,
    z_in_mm: u32,
    is_breakable: bool,
    is_perishable: bool,
    sender_info: Person,
    receiver_info: Person,
    id: u32,
}

impl Order {
    fn display(&self) -> Html {
        html!{
            <>
                {"send_date: "} {self.send_date.clone()} <br />
                {"x_in_mm: "} {self.x_in_mm.clone()} <br />
                {"y_in_mm: "} {self.y_in_mm.clone()} <br />
                {"z_in_mm: "} {self.z_in_mm.clone()} <br />
                {"is_breakable: "} {self.is_breakable.clone()} <br />
                {"is_perishable: "} {self.is_perishable.clone()} <br />
                {"sender_info: "} <br />
                <div style="margin-left: 20px;">
                    {self.sender_info.display()}
                </div>
                {"receiver_info: "} <br />
                <div style="margin-left: 20px;">
                    {self.receiver_info.display()}
                </div>
                <br /> <br />
            </>
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Orders {
    orders: Vec<Order>
}

async fn get_orders() -> Result<Orders, String> {
    log!("halloo??");
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
        204 => Ok(Orders { orders: vec![] }),
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
        Ok(Orders{orders}) => html! {
            <div>
                <a href="/"> {"Home"} </a> <br />
                <h1> {"Deliveries"} </h1>
                <ul>
                    { for orders.iter().map(|d| d.display()) }
                </ul>
            </div>
        },
        Err(e) => html! {
            <div>
                <h1> {"Error when getting orders"} </h1>
                <p> {e} </p>
            </div>
        }
    }
}

pub fn orders_page() -> Html {
    html!{
        <>
            <h1> {"Web shop orders"} </h1>
            <OrdersList />
        </>
    }
}