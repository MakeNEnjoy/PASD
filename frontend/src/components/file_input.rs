use gloo_console::log;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, InputEvent, File, Blob};
use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt, prelude::Closure};

fn get_target_from_input_event(e: InputEvent) -> HtmlInputElement {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    event_target.dyn_into().unwrap_throw()
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Callback<File>
}

#[function_component(FileInput)]
pub fn file_input(props: &Props) -> Html {
    let Props { on_change } = props.clone();

    let onchange = Callback::from(move |input_event: InputEvent| {
        let on_change = on_change.clone();
        let target = get_target_from_input_event(input_event.clone());
        let files = target.files().unwrap_throw();
        let file = files.get(0).unwrap_throw();
        on_change.emit(file);
    });

    html! {
        <>
            <input type="file" oninput={onchange} />
        </>
    }
}