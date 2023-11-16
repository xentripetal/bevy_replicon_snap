use bevy::prelude::Transform;
use crate::Interpolate;

impl Interpolate for Transform {
    fn interpolate(&self, other: Self, t: f32) -> Self {
        self.translation.lerp(other.translation, t);
        self.rotation.lerp(other.rotation, t);
        self.scale.lerp(other.scale, t);
        self.clone()
    }
}