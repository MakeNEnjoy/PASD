use diesel::SqliteConnection;
use diesel::prelude::*;
use log::{debug, info};
use crate::db::schema::deliveries;
use crate::db::DbError;
use crate::db::models::{Delivery, InsertableDelivery};


/// This function inserts a new delivery into the database.
///
///
/// Arguments:
/// * `conn`: &mut SqliteConnection - The connection to the database
/// * `delivery`: Delivery - The delivery to insert into the database
///
/// Returns:
/// A Result<Id, DbError>
pub fn insert_delivery(conn: &mut SqliteConnection, delivery: InsertableDelivery) -> Result<i32, DbError> {
    let result = diesel::insert_into(deliveries::table)
        .values(&delivery)
        .returning(deliveries::id)
        .get_result::<Option<i32>>(conn);

    match result.unwrap() {
        None => Err(DbError::from("Internal server error!")),
        Some(id) => Ok(id)
    }
}
