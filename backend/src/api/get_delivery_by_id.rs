//! this module contains a get_by_id function
use actix_web::{Error, HttpResponse};
use actix_web::web::{block, Data, Path};
use crate::{db::actions, DbPool};

/// This function fetches and returns a delivery from the database by id.
///
/// Arguments:
/// * `pool`: `Data<DbPool>` - The database connection pool.
/// * `id`: `Path<i32>` - if to be retrieved
///
/// Returns:
///
/// A `Result<HttpResponse, Error>`
pub async fn get_delivery_by_id(
    pool: Data<DbPool>,
    id: Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let result = block(move || {
        let mut conn = pool.get()?;
        actions::get_delivery_by_id(&mut conn, id)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match result {
        Some(deliveries) => Ok(HttpResponse::Ok().json(deliveries)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}
