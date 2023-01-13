use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod logistics_worker_app;
mod customer_app;
use crate::{
    router::Route,
    logistics_worker_app::logistics_worker_page,
    customer_app::customer_page
};

mod components;
use crate::components::{
    deliveries::Deliveries,
    create_delivery::create_delivery_page,
    delivery_info::DeliveryInfo,
    update_delivery::update_delivery_page,
    update_delivery_logistics::update_delivery_logistics_page,
    update_preferred_delivery_date::update_preferred_delivery_page,
    orders::orders_page,
    create_delivery_customer::create_delivery_customer_page,
    create_webshop_delivery::create_webshop_delivery_page,
};

fn main_page() -> Html {
    html!{
        <>
            <h1> {"Disruptive Delivery"} </h1>
            <a href="/logistics"> {"Page for Logistics Worker"} </a> <br />
            <a href="/customers"> {"Page for Package Senders and Receivers"} </a> <br />
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => main_page(),
        Route::Deliveries => html!{ <Deliveries /> },
        Route::CreateDelivery => create_delivery_page(),
        Route::UpdateDelivery { id } => update_delivery_page(id),
        Route::Delivery { id } => html!{ <DeliveryInfo id={id} /> },
        Route::LogisticsWorker => logistics_worker_page(),
        Route::UpdateStatus { id } => update_delivery_logistics_page(id),
        Route::UpdatePreferredDeliveryDate { id } => update_preferred_delivery_page(id),
        Route::Customers => customer_page(),
        Route::Orders => orders_page(),
        Route::CreateDeliveryCustomer => create_delivery_customer_page(),
        Route::MakeBid { id } => create_webshop_delivery_page(id)
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}