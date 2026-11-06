use bevy::{
    prelude::*,
    audio,
};

use crate::resources::Volume;

pub fn audio_setup(
    mut global_volume: ResMut<GlobalVolume>,
    volume: Res<Volume>,
) {
    // Volume takes a f32, usually between 0.0 (mute) and 1.0 (max)
    global_volume.volume = audio::Volume::Linear(volume.0 as f32 / 10.); 
}
