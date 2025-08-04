use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct SliderBar;
#[derive(Component)]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub value: f32
}

impl Slider {
    pub fn set_value(&mut self, x: f32) {
        self.value = f32::clamp(x, 0.0, 1.0);
    }
    #[allow(dead_code)]
    pub fn get_absolute_value(&self) -> f32 {
        self.min + (self.max - self.min) * self.value
    }
}

impl Default for Slider {
    fn default() -> Self {
        Slider { min: 0.0, max: 1.0, value: 0.0 }
    }
}