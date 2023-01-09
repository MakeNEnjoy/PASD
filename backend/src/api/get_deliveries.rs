use actix_web::{Error, HttpResponse};
use actix_web::web::{block, Data, Query};
use crate::{db::actions, DbPool};
use serde::Deserialize;

/// This struct represents the query elements accepted by the `get_deliveries` function.
/// Properties:
///
/// * `status`: The status of the package to filter on
#[derive(Deserialize)]
pub struct Status {
    status: Option<String>
}

/// This function returns all existing deliveries in the database, filtered by status
///
/// Arguments:
/// * `pool`: Data<DbPool> - The database connection pool.
/// * `query`: Status - status to filter on
/// * `req`: HttpRequest.
///
/// Returns:
///
/// A Result<HttpResponse, Error>
pub async fn get_deliveries(
    pool: Data<DbPool>,
    query: Query<Status>,
) -> Result<HttpResponse, Error> {
    let status = query.into_inner().status;
    let result = block(move || {
        let mut conn = pool.get()?;
        actions::get_deliveries(&mut conn, status)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match result {
        Some(deliveries) => Ok(HttpResponse::Created().json(deliveries)),
        None => Ok(HttpResponse::NoContent().finish())
    }
}
