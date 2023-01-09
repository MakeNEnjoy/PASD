use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::*;


#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = deliveries)]
pub struct Delivery {
    pub id: i32,
    pub origin_address: String,
    pub delivery_address: String,
    pub preferred_pickup: String,
    pub expected_pickup: String,
    pub preferred_delivery: String,
    pub expected_delivery: String,
    pub status: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = deliveries)]
pub struct OptionalDelivery {
    pub origin_address: Option<String>,
    pub delivery_address: Option<String>,
    pub preferred_pickup: Option<String>,
    pub expected_pickup: Option<String>,
    pub preferred_delivery: Option<String>,
    pub expected_delivery: Option<String>,
    pub status: Option<String>, //fixme: rember to check status on incoming struct
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = deliveries)]
pub struct InsertableDelivery {
    pub origin_address: Option<String>,
    pub delivery_address: String,
    pub preferred_pickup: Option<String>,
    pub expected_pickup: Option<String>,
    pub preferred_delivery: Option<String>,
    pub expected_delivery: Option<String>,
    pub status: String, //fixme: rember to check status on incoming struct
}