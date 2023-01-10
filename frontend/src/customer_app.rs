use yew::prelude::*;
use crate::components::navigate_to_delivery::NavigateToDelivery;

pub fn customer_page() -> Html {
    html!{
        <>
            <h1> {"Disruptive Delivery External Interface"} </h1>
            <a href="/create-delivery"> {"Create Delivery"} </a> <br />
            <NavigateToDelivery />
        </>
    }
}