use diesel::prelude::*;

pub enum FeatureState {
    On,
    Off,
}

impl Default for FeatureState {
    fn default() -> Self {
        FeatureState::Off
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::persistence::schema::features)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feature {
    pub id: String, // TODO figure out a nice way to use the uuid type from diesel or uuid crate
    pub name: String,
    pub state: bool,
}
