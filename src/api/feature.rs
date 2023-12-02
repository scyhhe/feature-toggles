use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use crate::model::feature::FeatureState;
use crate::persistence::persistence::Repository;
use crate::{model, persistence};

use super::api_error::ApiError;

#[get("/features/{name}")]
async fn lookup(db: web::Data<Repository>, name: web::Path<String>) -> HttpResponse {
    let result = db.lookup(&name).await;

    match result {
        Some(feature) => {
            let response = map_to_response(&feature);
            HttpResponse::Ok().json(response)
        }
        None => {
            let err_message = format!("Feature {} not found", &name);
            let err = ApiError::new(404, err_message);
            HttpResponse::NotFound().json(err)
        }
    }
}

#[post("/features")]
async fn create(
    db: web::Data<Repository>,
    feature_json: web::Json<model::feature::Feature>,
) -> HttpResponse {
    let f = feature_json.into_inner();
    let new_feature = persistence::feature::Feature {
        id: Uuid::new_v4().to_string(),
        name: f.name.clone(),
        state: f.is_on(),
    };

    db.create(&new_feature).await;
    HttpResponse::Ok().json(map_to_response(&new_feature))
}

// TODO clearly separate domain and api semantics
fn map_to_response(feature: &persistence::feature::Feature) -> model::feature::Feature {
    let name = &feature.name;
    let state = match feature.state {
        true => FeatureState::On,
        false => FeatureState::Off,
    };
    model::feature::Feature::new(name, Some(state))
}
