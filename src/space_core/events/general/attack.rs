use bevy::{math::Vec3, prelude::Entity};

use crate::space_core::{components::{health::DamageModel, inventory_item::{CombatSoundSet, CombatType}}, resources::doryen_fov::Vec3Int};

pub struct Attack {
    pub attacker_entity : Entity,
    pub attacker_sensed_by : Vec<Entity>,
    pub attacker_sensed_by_cached : Vec<Entity>,
    pub attacker_name : String,
    pub weapon_entity : Option<Entity>,
    pub weapon_name : String,
    pub weapon_a_name : String,
    pub targetted_limb : String,
    pub attacker_position : Vec3,
    pub angle : f32,
    pub damage_model : DamageModel,
    pub range : f32,
    pub combat_type : CombatType,
    pub combat_sound_set : CombatSoundSet,
    pub offense_words : Vec<String>,
    pub trigger_words : Vec<String>,
    pub targetted_entity : Option<Entity>,
    pub targetted_cell : Option<Vec3Int>,
}
