use actix_web::{guard, web};

/// This function configures the routes for the API
///
/// Arguments:
///
/// * `cfg`: &mut web::ServiceConfig - This is the configuration object that is used to configure the routes
pub fn config_services(cfg: &mut web::ServiceConfig) {
    //todo: this
    /*
    cfg.service(
        web::scope("/songs")
            .service(
                web::resource("")
                    .route(web::post().to(post_song::post_song_json)
                        .guard(guard::Not(guard::Header("content-type", "text/csv"))))
                    .route(web::post().to(post_song::post_song_csv)),
            )
            .service(
                web::scope("/{songID}")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_song_by_id::get_song_by_id))
                            .route(web::put().to(update_song_by_id::update_song_by_id_json)
                                .guard(guard::Not(guard::Header("content-type", "text/csv"))))
                            .route(web::put().to(update_song_by_id::update_song_by_id_csv))
                            .route(web::delete().to(delete_song_by_id::delete_song_by_id)),
                    )
            )
    );
    cfg.service(
        web::scope("/search")
            .service(
                web::scope("/songs")
                    .service(
                        web::resource("")
                            .route(web::get().to(search_songs::search_songs_by_name))
                    )
            )
            .service(
                web::scope("/artists")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_discography::get_discography_by_name))
                            .route(web::delete().to(delete_discography::delete_discography_by_name))
                    )
            )
    );
    cfg.service(
        web::scope("/artists/{ArtistID}")
            .service(
                web::resource("")
                    .route(web::get().to(get_discography::get_discography_by_id))
                    .route(web::delete().to(delete_discography::delete_discography_by_id))
            )
    );
    cfg.service(
        web::scope("/summary")
            .service(
                web::scope("/artistsearch")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_summary::get_summary_by_name))
                    )
            )
            .service(
                web::scope("/artistid/{artistID}")
                    .service(
                        web::resource("")
                            .route(web::get().to(get_summary::get_summary_by_id))
                    )
            )
    );
    cfg.service(
        web::scope("/popularity")
            .service(
                web::resource("")
                    .route(web::get().to(get_popular::get_popular))
            )
    );
*/
}