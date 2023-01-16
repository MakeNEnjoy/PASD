use web_sys::File;
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::{
    Request,
    FormData,
};
use gloo_console::log;
use serde_json::{to_string, from_str};
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use super::text_input::TextInput;
use super::file_input::FileInput;
use crate::router::Route;

struct Label {
    label: Option<File>,
}

impl Label {
    async fn post(label: File, id: u32) -> Result<String, String> {
        log!("sending: {}", &label);
        // let mut form = Form::new();
        // form.part("labelFile", Part::text(label.clone()).file_name("TABLEA.pdf"));
        let form = FormData::new().unwrap();
        form.set_with_blob("labelFile", &label).unwrap();

        let response = Request::post(&format!("/webshop/api/label?delivery_id={}", id))
            // .header("Content-Type", "multipart/form-data")
            .header("x-api-key", "46XiHoFBHG7sViWGTx7a")
            .body(form)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let body = response
            .text()
            .await
            .map_err(|e| e.to_string())?;
        log!("status: {}", response.status());
        log!("res: {}", &body);
        match response.status() {
            200 => Ok(body),
            _ => Err(body)
        }
    }

    fn send_label(&self, ctx: &Context<Self>) {
        let label = self.label.clone();
        let id = ctx.props().delivery_id;
        let navigator = ctx.link().navigator().unwrap();
        log!("sending label!!");
        wasm_bindgen_futures::spawn_local(async move {
            // Self::post(label.unwrap(), id).await.unwrap();
            match Self::post(label.unwrap(), id).await {
                Ok(_) => navigator.push(&Route::Deliveries),
                Err(e) => log!("Error: {}", e)
            }
        });

    }
}

enum Msg {
    CreateLabel,
    UpdateLabel(File)
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    pub delivery_id: u32,
}

impl Component for Label {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            label: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateLabel => {
                self.send_label(ctx);
                true
            },
            Msg::UpdateLabel(label) => {
                log!("label updated: {}", &label);
                self.label = Some(label);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                
                <label> { "Label" } </label>
                // <FileInput on_change={}/>
                <FileInput
                    on_change={ctx.link().callback(Msg::UpdateLabel)}
                />
                <button 
                    onclick={ctx.link().callback(|_| Msg::CreateLabel)}
                >
                    { "Create label" }
                </button>
            </div>
        }
    }
}


pub fn create_label_page(delivery_id: u32) -> Html {
    html! {
        <>
            <a href="/deliveries"> { "Back" } </a>
            <h1> { "Create label " } </h1>
            <Label delivery_id={delivery_id} />
        </>
    }
}