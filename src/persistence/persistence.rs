use diesel::{Connection, PgConnection, QueryDsl, RunQueryDsl};

use super::feature::Feature;
use crate::persistence::schema::features::dsl::*;

pub struct Repository<'a> {
    pool: &'a mut PgConnection, // TODO reuse the pool instead of creating a new one each time
}

impl Repository<'_> {
    pub fn new() -> Self {
        match dotenvy::dotenv() {
            Ok(value) => println!("loaded {:?}", value),
            Err(err) => panic!("fucked => {:?}", err),
        }

        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut conn: PgConnection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Failed to connect to postgres"));
        let pool = &mut conn;
        Repository { pool }
    }

    pub fn lookup(&self, feature_name: &str) -> Option<Feature> {
        let maybe_feature = features.find(id).get_result(&mut self.pool);

        match maybe_feature {
            Ok(feature) => Some(feature),
            Err(_) => None,
        }
    }
}
