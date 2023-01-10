use gloo_console::log;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, InputEvent};
use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

fn get_target_from_input_event(e: InputEvent) -> HtmlInputElement {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    event_target.dyn_into().unwrap_throw()
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let target = get_target_from_input_event(e);
    target.value()
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        let value = get_value_from_input_event(input_event.clone());
        on_change.emit(value);
    });

    html! {
        <>
            <input type="text" {oninput} maxlength="30" />
        </>
    }
}

#[function_component(DateInput)]
pub fn date_input(props: &Props) -> Html {
    let Props { on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        let value = get_value_from_input_event(input_event.clone());
        on_change.emit(format!("{}:00", value));
    });

    html! {
        <>
            <input type="datetime-local" {oninput} maxlength="30" />
        </>
    }
}

fn get_value_from_dropdown(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlSelectElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[function_component(StatusInput)]
pub fn status_input(props: &Props) -> Html {
    let Props { on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        let value = get_value_from_dropdown(input_event.clone());
        on_change.emit(value);
    });

    html! {
        <select {oninput} >
            <option value="delivered"> {"Delivered"} </option>
            <option value="in transit"> {"In Transit"} </option>
            <option value="in warehouse"> {"In Warehouse"} </option>
            <option value="awaiting pickup"> {"Awaiting Pickup"} </option>
        </select>
    }
}