use web_sys::{Event, HtmlInputElement, InputEvent};
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
        on_change.emit(value);
    });

    html! {
        <>
            <input type="datetime-local" {oninput} maxlength="30" />
        </>
    }
}

#[function_component(StatusInput)]
pub fn status_input(props: &Props) -> Html {
    let Props { on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        let value = get_value_from_input_event(input_event.clone());
        on_change.emit(value);
    });

    html! {
        <>
            <input type="text" {oninput} required=true maxlength="30" />
        </>
    }
}