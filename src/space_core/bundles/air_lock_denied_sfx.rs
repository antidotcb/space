
use bevy::prelude::{Transform};

use crate::space_core::components::{entity_data::{EntityData}, entity_updates::EntityUpdates, sensable::Sensable, sfx::Sfx, static_transform::StaticTransform};

pub struct AirLockDeniedSfxBundle;

pub const PLAY_BACK_DURATION : f32 = 1.5 + 1.;

impl AirLockDeniedSfxBundle {

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
            unit_db: 19.,
            unit_size: 1.,
            stream_id: "doorAccessDenied".to_string(),
            play_back_duration: PLAY_BACK_DURATION,
            ..Default::default()
        },
        EntityUpdates::default(),
    )

    }

}
