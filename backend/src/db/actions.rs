//! this module defines actions available on the database
use diesel::SqliteConnection;
use diesel::prelude::*;
use log::info;
use crate::db::schema::deliveries;
use crate::db::DbError;
use crate::db::models::{Delivery, Id, InsertableDelivery, OptionalDelivery};


/// This function inserts a new delivery into the database.
///
///
/// Arguments:
/// * `conn`: &mut SqliteConnection - The connection to the database
/// * `delivery`: Delivery - The delivery to insert into the database
///
/// Returns:
/// A Result<Id, DbError>
pub fn insert_delivery(conn: &mut SqliteConnection, delivery: InsertableDelivery) -> Result<Id, DbError> {
    let result = diesel::insert_into(deliveries::table)
        .values(&delivery)
        .returning(deliveries::id)
        .get_result::<i32>(conn)?;
    Ok(Id { id: result })
}

/// This function fetches existing deliveries from the database.
///
///
/// Arguments:
/// * `conn`: `&mut SqliteConnection` - The connection to the database
/// * `status`: `Option<String>` - status to filter on
///
/// Returns:
/// A `Result<Option<Vec<Delivery>>`, DbError>
pub fn get_deliveries(conn: &mut SqliteConnection, status: Option<String>) -> Result<Option<Vec<Delivery>>, DbError> {
    let mut query = deliveries::table.into_boxed();
    if let Some(status) = status {
        query = query.filter(deliveries::status.eq(status.clone()));
        if status.eq("awaiting pickup") {
            query = query.order(deliveries::preferred_pickup.asc());
        } else {
            query = query.order(deliveries::preferred_delivery.asc());
        }
    } else {
        query = query.order(deliveries::preferred_delivery.asc());
    }

    let deliveries = query.load::<Delivery>(conn)?;
    if deliveries.is_empty() {
        Ok(None)
    } else {
        Ok(Some(deliveries))
    }
}

/// This function fetches an existing delivery from the database by id.
///
///
/// Arguments:
/// * `conn`: &mut SqliteConnection - The connection to the database
/// * `id`: i32 - the id of the delivery to retrieve
///
/// Returns:
/// A `Result<Option<Delivery>`, DbError>
pub fn get_delivery_by_id(conn: &mut SqliteConnection, id: i32) -> Result<Option<Delivery>, DbError> {
    let delivery = deliveries::table
        .filter(deliveries::id.eq(id))
        .first::<Delivery>(conn)
        .optional()?;
    Ok(delivery)
}

/// This function updates an existing delivery from the database by id.
///
///
/// Arguments:
/// * `conn`: &mut SqliteConnection - The connection to the database
/// * `id`: i32 - The id of the delivery to be updated
/// * `delivery`: OptionalDelivery - struct representing the fields to be updated
///
/// Returns:
/// A `Result<Option<Delivery>`, DbError>
pub fn update_delivery_by_id(conn: &mut SqliteConnection, id: i32, delivery: OptionalDelivery) -> Result<Delivery, DbError> {
    let result = diesel::update(deliveries::table.filter(deliveries::id.eq(id)))
        .set::<OptionalDelivery>(delivery)
        .get_result::<Delivery>(conn);
    Ok(result.unwrap())
}

/// This function deletes an existing delivery from the database by id.
///
/// Arguments:
/// * `conn`: &mut SqliteConnection - The connection to the database
/// * `id`: i32 - the id of the delivery to delete
///
/// Returns:
/// A Result<(), DbError>
pub fn delete_delivery_by_id(conn: &mut SqliteConnection, id: i32) -> Result<(), DbError> {
    let n = diesel::delete(deliveries::table
        .filter(deliveries::id.eq(id)))
        .execute(conn);
    info!("deleting {} rows!", n.unwrap());
    Ok(())
}