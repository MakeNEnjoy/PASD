use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use super::text_input::TextInput;

#[function_component(NavigateToDelivery)]
pub fn navigate_to_delivery() -> Html {
    let id = use_state(|| "".to_string());
    let on_change = {
        let id = id.clone();
        Callback::from(move |value: String| {
            id.set(value);
        })
    };
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Delivery { id: (*id).parse().unwrap() })
    });
    html! {
        <>
            <TextInput on_change={on_change} />
            <button {onclick}> { "Get tracking info" } </button>
        </>
    }
}