pub mod models;
pub mod routes;

use actix_web::web::ServiceConfig;

// Add all the routes to the app using a ServiceConfig pattern,
pub fn config_app() -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.service(routes::hello)
            .service(routes::get_samples)
            .service(routes::get_sample);
    })
}
