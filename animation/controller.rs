use super::clip::Clip;
use super::pose::Pose;
use super::skeleton::Skeleton;
use math::mat4::Mat4;

#[derive(Clone)]
pub struct AnimationController {
    clips: Vec<Clip>,
    current_clip_index: Option<usize>,
    current_time: f32,
    is_playing: bool,
    current_pose: Pose,
    skeleton: Skeleton,
}

impl AnimationController {
    pub fn new(skeleton: Skeleton) -> Self {
        Self {
            clips: Vec::new(),
            current_clip_index: None,
            current_time: 0.0,
            is_playing: false,
            current_pose: skeleton.rest_pose.clone(),
            skeleton,
        }
    }

    pub fn add_clip(&mut self, clip: Clip) -> usize {
        self.clips.push(clip);
        self.clips.len() - 1
    }

    pub fn play(&mut self, clip_index: usize) {
        if clip_index < self.clips.len() {
            self.current_clip_index = Some(clip_index);
            self.current_time = 0.0;
            self.is_playing = true;
            self.current_pose = self.skeleton.rest_pose.clone();
        } else {
            println!("Clip index {} out of bounds", clip_index);
        }
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_time = 0.0;
        self.current_pose = self.skeleton.rest_pose.clone();
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn resume(&mut self) {
        self.is_playing = true;
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.is_playing {
            return;
        }

        if let Some(clip_index) = self.current_clip_index {
            self.current_time += delta_time;
            self.current_pose = self.skeleton.rest_pose.clone();
            self.clips[clip_index].sample(&mut self.current_pose, self.current_time);
        }
    }

    pub fn get_pose_matrices(&self) -> Vec<Mat4> {
        let mut final_mats = Vec::new();
        let len = self.skeleton.rest_pose.joints.len();
        final_mats.resize(len, Mat4::IDENTITY);

        let pose = if self.is_playing {
            &self.current_pose
        } else {
            &self.skeleton.rest_pose
        };

        for i in 0..len {
            if let Some(inverse_pose) = self.skeleton.inverse_bind_pose[i] {
                let world = pose.get_global_tranform(i);
                final_mats[i] = world.to_mat() * inverse_pose;
            }
        }

        final_mats
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn current_clip(&self) -> Option<&Clip> {
        self.current_clip_index.map(|i| &self.clips[i])
    }

    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }
}
