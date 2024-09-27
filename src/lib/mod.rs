pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// Reexport some frequently used type to crate root
pub use data::DataError;
pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::time::Time;
pub use domain::Clip;
pub use service::ServiceError;

use data::AppDatabase;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use web::hit_counter::HitCounter;
use web::renderer::Renderer;

// Build the Rocket server
pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
    pub hit_counter: HitCounter,
}

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .manage::<HitCounter>(config.hit_counter)
        .mount("/", web::http::routes())
        .mount("/api/clip", web::api::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
        .register("/api/clip", web::api::catcher::catchers())
}
