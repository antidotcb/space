
use bevy::prelude::{Transform};

use crate::space_core::components::{entity_data::{EntityData}, entity_updates::EntityUpdates, sensable::Sensable, sfx::{Sfx, get_random_pitch_scale}, static_transform::StaticTransform};

pub struct AirLockOpenSfxBundle;

pub const PLAY_BACK_DURATION : f32 = 4.5 + 1.;

impl AirLockOpenSfxBundle {

    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        Sfx,
        EntityUpdates
    ) {


        (StaticTransform {
            transform: passed_transform,
        },
        EntityData {
            entity_class : "SFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        Sfx {
            unit_db: 13.,
            stream_id: "doorOpen".to_string(),
            play_back_duration: PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.6),
            ..Default::default()
        },
        EntityUpdates::default(),
    )

    }

}
