
pub struct RepeatingSfx {
    pub area_mask : u8,
    pub attenuation_filter_cutoff_hz : f32,	
    pub attenuation_filter_db : f32,
    pub attenuation_model : u8,
    pub auto_play : bool,	
    pub bus : String,
    pub doppler_tracking : u8,
    pub emission_angle_degrees : f32,
    pub emission_angle_enabled : bool,
    pub emission_angle_filter_attenuation_db : f32,
    pub max_db : f32,
    pub max_distance : f32,
    pub out_of_range_mode : u8,
    pub pitch_scale : f32,
    pub playing : bool,
    pub stream_paused : bool,
    pub unit_db : f32,
    pub unit_size : f32,
    pub stream_id : String,
    pub auto_destroy : bool,
    pub repeat_time : f32
}

impl Default for RepeatingSfx {
    fn default() -> Self {
        Self {
            area_mask : 0,
            attenuation_filter_cutoff_hz: 5000.,
            attenuation_filter_db : -24.,
            attenuation_model: 0,
            auto_play : true,
            bus : "Master".to_string(),
            doppler_tracking : 0,
            emission_angle_degrees : 45.,
            emission_angle_enabled: false,
            emission_angle_filter_attenuation_db : -12.,
            max_db : 3.,
            max_distance: 0.,
            out_of_range_mode : 0,
            pitch_scale: 1.,
            playing : false,
            stream_paused : false,
            unit_db: 0.,
            unit_size: 1.,
            stream_id: "".to_string(),
            auto_destroy : true,
            repeat_time: 1.,
        }
    }
}
