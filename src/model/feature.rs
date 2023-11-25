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
    name: String,
    state: FeatureState,
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

    pub fn toggle(&mut self) -> Self {
        Self {
            id: self.id,
            name: self.name.to_owned(),
            state: match self.state {
                FeatureState::Off => FeatureState::On,
                FeatureState::On => FeatureState::Off,
            },
        }
    }
}
