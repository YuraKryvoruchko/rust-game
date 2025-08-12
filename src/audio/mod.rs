use bevy::prelude::*;
use bevy::{audio::Volume};
use core::fmt::Display;

pub trait Volumable {
    fn get_volume(self) -> f32;
}

#[derive(Resource, Clone, Copy)]
pub struct MusicVolume(pub f32);

impl Volumable for MusicVolume {
    fn get_volume(self) -> f32 {
        self.0
    }
}

impl Display for MusicVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

#[derive(Resource, Clone, Copy)]
pub struct SoundVolume(pub f32);

impl Volumable for SoundVolume {
    fn get_volume(self) -> f32 {
        self.0
    }
}

impl Display for SoundVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

#[derive(Component)]
pub struct Music;
#[derive(Component)]
pub struct Sound;

pub fn volume_system<T, R>(
    music_controllers: Query<&mut AudioSink, With<T>>,
    music_volume: Res<R>
) where T: Component,
        R: Resource + Volumable + Copy 
{
    for mut sink in music_controllers {
        sink.set_volume(Volume::Linear(music_volume.get_volume() / 100.0));
    }
}