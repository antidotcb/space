use bevy::prelude::Transform;

use crate::space_core::components::{entity_data::EntityData, entity_updates::EntityUpdates, sensable::Sensable, sfx::{Sfx, get_random_pitch_scale}, static_transform::StaticTransform};

pub struct LaserLightHit3Bundle;

pub const LASER_LIGHT_HIT3_PLAY_BACK_DURATION : f32 = 1.8 + 1.;

impl LaserLightHit3Bundle {
    
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
            unit_db: 25.,
            unit_size: 1.,
            stream_id: "laser_light_hit3".to_string(),
            play_back_duration: LASER_LIGHT_HIT3_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },
        EntityUpdates::default(),)

    }

}
