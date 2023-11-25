use actix_web::{get, web, HttpResponse};

use crate::model;
use crate::model::feature::FeatureState;
use crate::persistence::{feature::Feature, persistence::Repository};

#[get("/features/{name}")]
async fn lookup(db: web::Data<Repository<'_>>, name: web::Path<String>) -> HttpResponse {
    let result = db.lookup(&name);

    match result {
        Some(feature) => {
            let response = map_to_response(&feature);
            HttpResponse::Ok().json(response)
        }
        None => HttpResponse::NotFound().body("Feature not found"),
    }
}

// TODO clearly separate domain and api semantics
fn map_to_response(feature: &Feature) -> model::feature::Feature {
    let name = &feature.name;
    let state = match feature.state {
        true => FeatureState::On,
        false => FeatureState::Off,
    };
    return model::feature::Feature::new(&feature.name, Some(state));
}
