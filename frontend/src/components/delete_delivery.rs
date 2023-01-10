use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use reqwasm::http::Request;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[function_component(DeleteDelivery)]
pub fn delete_song(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let id = props.id.clone();
    let onclick = Callback::from(move |_| {
        let id = id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            Request::delete(format!("/api/deliveries/{}", id).as_str())
                .send()
                .await
                .unwrap();
        });

        navigator.push(&Route::Deliveries)
    });
    html!{
        <button {onclick}> { "Delete" } </button>
    }
}