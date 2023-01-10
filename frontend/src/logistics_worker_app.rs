use yew::prelude::*;

pub fn logistics_worker_page() -> Html {
    html!{
        <>
            <h1> {"Disruptive Delivery Internal Interface"} </h1>
            <a href="/deliveries"> {"Deliveries"} </a> <br />
            <a href="/orders"> {"Orders"} </a> <br />
        </>
    }
}