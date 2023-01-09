use actix_web::{Error, HttpResponse};
use actix_web::web::{block, Data, Path};
use crate::{db::actions, DbPool};

/// This function deletes a delivery by its unique id.
///
/// Arguments:
/// * `pool`: `Data<DbPool>` - The database connection pool.
/// * 'id': `Query<i32>` - the id of the delivery to be deleted.
///
/// Returns:
///
/// A `Result<HttpResponse, Error>`
pub async fn delete_delivery_by_id(
    pool: Data<DbPool>,
    id: Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();

    let _ = block(move || {
        let mut conn = pool.get()?;
        actions::delete_delivery_by_id(&mut conn, id)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
