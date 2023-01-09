use actix_web::{Error, HttpResponse};
use actix_web::web::{block, Data, Json};
use crate::{db::actions, DbPool};
use crate::db::models::InsertableDelivery;

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
    if !vec!["awaiting pickup", "in warehouse", "in transit", "delivered"].contains(&&*delivery.status) {   //todo: rember to implement this for PUT as well
        return Ok(HttpResponse::BadRequest().body(format!("invalid status '{}'.\n", delivery.status)));
    }

    let result = block(move || {
        let mut conn = pool.get()?;
        actions::insert_delivery(&mut conn, delivery)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(result))
}
