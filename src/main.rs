use actix_web::{web::Data, App, HttpServer};
use api::feature::{create, lookup};
use persistence::persistence::Repository;

mod api;
mod model;
mod persistence;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let repository = Repository::new();
        let app_data = Data::new(repository);
        App::new()
            .app_data(Data::clone(&app_data))
            .service(lookup)
            .service(create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
