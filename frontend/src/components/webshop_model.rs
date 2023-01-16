use serde::{Serialize, Deserialize};
use super::navigate_to_bidder::NavigateToBidder;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub street_and_number: String,
    pub zipcode: String,
    pub city: String,
    pub country: String
}

impl Person {
    pub fn display(&self) -> Html {
        html!{
            <>
                {"name: "} {self.name.clone()} <br />
                {"street_and_number: "} {self.street_and_number.clone()} <br />
                {"zipcode: "} {self.zipcode.clone()} <br />
                {"city: "} {self.city.clone()} <br />
                {"country: "} {self.country.clone()} <br />
            </>
        }
    }
    pub fn address(&self) -> String {
        format!("{} {} {} {}", self.street_and_number.clone(), self.zipcode.clone(), self.city.clone(), self.country.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub send_date: String,
    pub x_in_mm: u32,
    pub y_in_mm: u32,
    pub z_in_mm: u32,
    pub is_breakable: bool,
    pub is_perishable: bool,
    pub sender_info: Person,
    pub receiver_info: Person,
    pub id: u32,
}

impl Order {
    pub fn display(&self) -> Html {
        html!{
            <>
                // {"id: "} {self.id.clone()} <br />
                {"send_date: "} {self.send_date.clone()} <br />
                {"x_in_mm: "} {self.x_in_mm.clone()} <br />
                {"y_in_mm: "} {self.y_in_mm.clone()} <br />
                {"z_in_mm: "} {self.z_in_mm.clone()} <br />
                {"is_breakable: "} {self.is_breakable.clone()} <br />
                {"is_perishable: "} {self.is_perishable.clone()} <br />
                {"sender_info: "} <br />
                <div style="margin-left: 20px;">
                    {self.sender_info.display()}
                </div>
                {"receiver_info: "} <br />
                <div style="margin-left: 20px;">
                    {self.receiver_info.display()}
                </div>
                <NavigateToBidder
                    id={self.id}
                    delivery_address={self.receiver_info.address()}
                    pickup_address={self.sender_info.address()}
                />
                <br /> <br />

            </>
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Orders {
    pub orders: Vec<Order>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressQuery {
    pub delivery_address: String,
    pub pickup_address: String,
}