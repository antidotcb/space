use bevy::{math::Vec3, prelude::Color};

pub struct ReflectionProbe {
    pub projection_enabled : bool,
    pub cull_mask : i64,
    pub shadows_enabled : bool,
    pub extents : Vec3,
    pub intensity : f32,
    pub interior_ambient : Color,
    pub interior_ambient_probe_contribution : f32,
    pub interior_ambient_energy : f32,
    pub set_as_interior : bool,
    pub max_distance : f32,
    pub origin_offset : Vec3,
    pub update_mode : u8
}
