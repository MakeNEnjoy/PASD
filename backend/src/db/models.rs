//! this module contains structs used in the backend
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::schema::*;

///This struct models an id of a delivery.
#[derive(Serialize)]
pub struct Id {
    pub id: i32,
}

///This struct represents a delivery, retrieved from the DB.
#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = deliveries)]
pub struct Delivery {
    pub id: i32,
    pub webshop_id: Option<i32>,
    pub origin_address: Option<String>,
    pub delivery_address: String,
    pub preferred_pickup: Option<NaiveDateTime>,
    pub expected_pickup: Option<NaiveDateTime>,
    pub preferred_delivery: Option<NaiveDateTime>,
    pub expected_delivery: Option<NaiveDateTime>,
    pub status: String,
}

///This struct represents a delivery, to be inserted into the DB.
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = deliveries)]
pub struct InsertableDelivery {
    pub webshop_id: Option<i32>,
    pub origin_address: Option<String>,
    pub delivery_address: String,
    pub preferred_pickup: Option<NaiveDateTime>,
    pub expected_pickup: Option<NaiveDateTime>,
    pub preferred_delivery: Option<NaiveDateTime>,
    pub expected_delivery: Option<NaiveDateTime>,
    pub status: String,
}

///This struct represents a delivery, to be updated in the database
#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = deliveries)]
pub struct OptionalDelivery {
    pub origin_address: Option<String>,
    pub delivery_address: Option<String>,
    pub preferred_pickup: Option<NaiveDateTime>,
    pub expected_pickup: Option<NaiveDateTime>,
    pub preferred_delivery: Option<NaiveDateTime>,
    pub expected_delivery: Option<NaiveDateTime>,
    pub status: Option<String>,
}

impl OptionalDelivery {
    ///this function checks if an OptionalDelivery is empty
    pub fn is_empty(&self) -> bool {
        self.origin_address == None &&
            self.delivery_address == None &&
            self.preferred_pickup == None &&
            self.expected_pickup == None &&
            self.preferred_delivery == None &&
            self.expected_delivery == None &&
            self.status == None
    }
}