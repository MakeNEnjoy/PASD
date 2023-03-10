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
    UpdateDelivery {id: u32},
    #[at("/logistics")]
    LogisticsWorker,
    #[at("/update-status/:id")]
    UpdateStatus {id: u32},
    #[at("/update-date/:id")]
    UpdatePreferredDeliveryDate {id: u32},
    #[at("/customers")]
    Customers,
    #[at("/orders")]
    Orders,
    #[at("/create-delivery-customer")]
    CreateDeliveryCustomer,
    #[at("/make-bid/:id")]
    MakeBid {id: u32},
    #[at("/create-label/:id")]
    CreateLabel {id: u32},
}