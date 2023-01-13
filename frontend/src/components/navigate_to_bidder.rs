use yew::prelude::*;
use yew_router::prelude::*;
use super::webshop_model::AddressQuery;
use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: u32,
    pub delivery_address: String,
    pub pickup_address: String,
}

#[function_component(NavigateToBidder)]
pub fn navigate_to_bidder(props: &Props) -> Html {
    let Props { id, delivery_address, pickup_address } = props.clone();
    let navigator = use_navigator().unwrap();
    let onclick = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push_with_query(
                &Route::MakeBid { id },
                &AddressQuery {
                    delivery_address: delivery_address.clone(),
                    pickup_address: pickup_address.clone(),
                },
            ).unwrap();
        })
    };
    html! {
        <>
            <button {onclick}> { "Bid" } </button>
        </>
    }
}
