use bevy::prelude::*;

#[derive(Resource)]
pub struct Controller {
    pub is_paused: bool
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            is_paused: false
        }
    }
}