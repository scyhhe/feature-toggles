use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum FeatureState {
    On,
    Off,
}

impl Default for FeatureState {
    fn default() -> Self {
        FeatureState::Off
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    id: Uuid,
    pub name: String,
    pub state: FeatureState,
}

impl Feature {
    pub fn new(name: &str, state: Option<FeatureState>) -> Self {
        let id = Uuid::new_v4();
        let name = name.to_string();
        let state = state.unwrap_or_default();

        return Feature { id, name, state };
    }

    pub fn is_on(&self) -> bool {
        return if let FeatureState::On = self.state {
            true
        } else {
            false
        };
    }
}
