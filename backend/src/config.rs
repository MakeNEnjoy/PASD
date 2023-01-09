use actix_web::web;
use crate::api::{delete_delivery_by_id, get_deliveries, get_delivery_by_id, post_delivery, update_delivery_by_id};

/// This function configures the routes for the API
///
/// Arguments:
///
/// * `cfg`: &mut web::ServiceConfig - This is the configuration object that is used to configure the routes
pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/deliveries")
            .service(
                web::resource("")
                    .route(web::get().to(get_deliveries::get_deliveries))
                    .route(web::post().to(post_delivery::post_delivery)),
            )
            .service(
                web::scope("/{deliveryID}")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_delivery_by_id::get_delivery_by_id))
                            .route(web::put().to(update_delivery_by_id::update_delivery_by_id))
                            .route(web::delete().to(delete_delivery_by_id::delete_delivery_by_id)),
                    )
            )
    );
}