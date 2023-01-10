use yew::prelude::*;
use yew_router::prelude::*;

mod router;
use crate::router::Route;

mod components;
use crate::components::{
    deliveries::Deliveries,
    create_delivery::create_delivery_page,
    delivery_info::DeliveryInfo,
    update_delivery::update_delivery_page
};

fn main_page() -> Html {
    html!{
        <>
            <h1> {"Hello World"} </h1>
            <a href="/deliveries"> {"Deliveries"} </a> <br />
            <a href="/create-delivery"> {"Create Delivery"} </a> <br />
            <a href="/update-delivery"> {"Update Delivery"} </a>
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => main_page(),
        Route::Deliveries => html!{ <Deliveries /> },
        Route::CreateDelivery => create_delivery_page(),
        Route::Delivery { id } => html!{ <DeliveryInfo id={id} /> },
        Route::UpdateDelivery { id } => update_delivery_page(id),
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