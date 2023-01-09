use actix_web::{Error, HttpResponse};
use actix_web::web::{block, Data, Json, Path};
use crate::{db::actions, DbPool};
use crate::db::models::OptionalDelivery;

/// This function updates a delivery, then returns the updated song.
///
/// Arguments:
/// * `pool`: `Data<DbPool>` - The database connection pool.
/// * 'id': `Query<String>` - the id of the delivery to be updated.
/// * `delivery`: `Json<OptionalDelivery>` - The delivery object that will be updated into the database.
///
/// Returns:
///
/// A `Result<HttpResponse, Error>`
pub async fn update_delivery_by_id(
    pool: Data<DbPool>,
    id: Path<i32>,
    delivery: Json<OptionalDelivery>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let delivery = delivery.into_inner();
    if delivery.is_empty() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    if let Some(status) = delivery.status.clone() {
        if !vec!["awaiting pickup", "in warehouse", "in transit", "delivered"].contains(&&*status) {
            return Ok(HttpResponse::BadRequest().body(format!("invalid status '{}'.\n", status)));
        }
    }

    let result = block(move || {
        let mut conn = pool.get()?;
        actions::update_delivery_by_id(&mut conn, id, delivery)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
