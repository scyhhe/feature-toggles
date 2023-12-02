use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use super::feature::Feature;
use crate::persistence::schema::features::dsl::*;

pub struct Repository {} // TODO reuse the pool instead of creating a new one each time

impl Repository {
    fn establish_connection() -> PgConnection {
        match dotenvy::dotenv() {
            Ok(value) => println!("loaded {:?}", value),
            Err(err) => panic!("scuffed => {:?}", err),
        }

        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Failed to connect to postgres {}", database_url))
    }

    pub fn new() -> Self {
        Repository {}
    }

    pub async fn lookup(&self, feature_name: &str) -> Option<Feature> {
        let conn = &mut Self::establish_connection();
        let maybe_feature = features.filter(name.eq(feature_name)).first(conn);

        maybe_feature.ok()
    }

    pub async fn create(&self, feature: &Feature) -> () {
        let conn = &mut Self::establish_connection();
        let result = diesel::insert_into(features).values(feature).execute(conn);

        result.map(|_| ()).unwrap()
    }
}
