use diesel::SqliteConnection;
use diesel::prelude::*;
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
        .get_result::<i32>(conn);
    Ok(result.unwrap())
}

/// This function fetches existing deliveries from the database.
///
///
/// Arguments:
/// * `conn`: &mut SqliteConnection - The connection to the database
/// * `status`: Option<String> - status to filter on
///
/// Returns:
/// A Result<Option<Vec<Delivery>>, DbError>
pub fn get_deliveries(conn: &mut SqliteConnection, status: Option<String>) -> Result<Option<Vec<Delivery>>, DbError> {
    match status {
        None => {
            let deliveries = deliveries::table
                .load::<Delivery>(conn)?;

            if deliveries.is_empty() {
                Ok(None)
            } else {
                Ok(Some(deliveries))
            }
        }
        Some(status) => {
            let deliveries = deliveries::table
                .filter(deliveries::status.eq(status))
                .load::<Delivery>(conn)?;

            if deliveries.is_empty() {
                Ok(None)
            } else {
                Ok(Some(deliveries))
            }
        }
    }


}