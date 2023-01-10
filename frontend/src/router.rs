use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/deliveries")]
    Deliveries,
    #[at("/create-delivery")]
    CreateDelivery,
    #[at("/deliveries/:id")]
    Delivery {id: u32},
    #[at("/update-delivery/:id")]
    UpdateDelivery {id: u32}
}