use actix_web::{Error, HttpResponse};
use actix_web::web::{block, Data, Json};
use crate::{db::actions, DbPool};
use crate::db::models::{Delivery, InsertableDelivery};

/// This function takes a delivery, inserts it into the database, and returns the result
///
/// Arguments:
/// * `pool`: Data<DbPool> - The database connection pool.
/// * `delivery`: Delivery - The delivery object that will be inserted into the database.
/// * `req`: HttpRequest.
///
/// Returns:
///
/// A Result<HttpResponse, Error>
pub async fn post_delivery(
    pool: Data<DbPool>,
    delivery: Json<InsertableDelivery>,
) -> Result<HttpResponse, Error> {
    let delivery = delivery.into_inner();

    let result = block(move || {
        let mut conn = pool.get()?;
        actions::insert_delivery(&mut conn, delivery)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(result))
}
