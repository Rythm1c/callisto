use super::track;
use math::quaternion::Quat;
use math::transform::Transform;
use math::vec3::Vec3;

use super::frame::{QuaternionFrame, VectorFrame};

#[derive(Clone)]
pub struct TransformTrack {
    pub id: u32,
    pub position: track::VectorTrack,
    pub rotation: track::QuatTrack,
    pub scaling: track::VectorTrack,
}

impl TransformTrack {
    pub fn new() -> Self {
        Self {
            id: 0,
            position: track::VectorTrack::new(),
            rotation: track::QuatTrack::new(),
            scaling: track::VectorTrack::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.position.frames.len() > 1
            || self.rotation.frames.len() > 1
            || self.scaling.frames.len() > 1
    }

    pub fn resize(&mut self, size: usize) {
        for _ in 0..size {
            self.position.frames.push(VectorFrame::new());
            self.rotation.frames.push(QuaternionFrame::new());
            self.scaling.frames.push(VectorFrame::ONE);
        }
    }

    pub fn get_start_time(&self) -> f32 {
        let mut result = 0.0;
        let mut is_set = false;

        if self.position.frames.len() > 1 {
            result = self.position.get_start_time();
            is_set = true;
        }

        if self.rotation.frames.len() > 1 {
            let rotation_start = self.rotation.get_start_time();
            if rotation_start < result || !is_set {
                result = rotation_start;
                is_set = true;
            }
        }

        if self.scaling.frames.len() > 1 {
            let scale_start = self.scaling.get_start_time();
            if scale_start < result || !is_set {
                result = scale_start;
            }
        }

        result
    }

    pub fn get_end_time(&self) -> f32 {
        let mut result = 0.0;
        let mut is_set = false;

        if self.position.frames.len() > 1 {
            result = self.position.get_end_time();
            is_set = true;
        }

        if self.rotation.frames.len() > 1 {
            let rotation_end = self.rotation.get_end_time();
            if rotation_end > result || !is_set {
                result = rotation_end;
                is_set = true;
            }
        }

        if self.scaling.frames.len() > 1 {
            let scale_end = self.scaling.get_end_time();
            if scale_end > result || !is_set {
                result = scale_end;
            }
        }

        result
    }

    pub fn sample(&mut self, reference: &Transform, time: f32, looping: bool) -> Transform {
        let mut result = *reference;

        if self.position.frames.len() > 1 {
            result.translation = self.position.sample::<Vec3>(time, looping);
        }

        if self.rotation.frames.len() > 1 {
            result.orientation = self.rotation.sample::<Quat>(time, looping);
        }

        if self.scaling.frames.len() > 1 {
            result.scaling = self.scaling.sample::<Vec3>(time, looping);
        }

        result
    }
}
