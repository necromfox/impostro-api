use std::sync::Mutex;
use std::env;
use actix_web::{
    web::{self, Data},
    App, HttpServer, HttpResponse, HttpRequest,
};

pub mod api;
pub mod ws;

pub type DataTy = Data<Mutex<impostro_shared::ImpostroData>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let data = Data::new(Mutex::new(
        impostro_shared::ImpostroData::default(),
    ));

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .route("sessions", web::post().to(api::get_sessions_fn))
            .route("groups", web::post().to(api::get_groups_fn))
            .route("members", web::post().to(api::get_members_fn))
            .route("validate_session_id", web::post().to(api::validate_session_id_fn))
            .route("create_session", web::post().to(api::create_session_fn))
            .route("ws", web::get().to(api::index::index))
            .route("heartbeat", web::get().to(|_: HttpRequest| HttpResponse::Ok()))
    })
    .bind(("localhost", port))?
    .run()
    .await
}
