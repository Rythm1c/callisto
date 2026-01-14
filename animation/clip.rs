// _______________________________________________________________________________________________________
// _______________________________________________________________________________________________________
// yet again lots of help from "gabor szauer - hands on c++ game animation programming packt"

use super::pose::Pose;
use super::track_transform::TransformTrack;

#[derive(Clone)]
pub struct Clip {
    pub tracks: Vec<TransformTrack>,
    pub name: String,
    start_time: f32,
    end_time: f32,
    looping: bool,
}

impl Clip {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            name: String::from("None"),
            start_time: 0.0,
            end_time: 0.0,
            looping: true,
        }
    }

    pub fn sample(&mut self, out_pose: &mut Pose, time: f32) -> f32 {
        if self.get_duration() == 0.0 {
            return 0.0;
        }

        let time = self.adjust_time_to_fit_range(time);

        let len = self.tracks.len();
        for i in 0..len {
            let j = self.tracks[i].id;
            let local = &out_pose.joints[j as usize];
            let animated = self.tracks[i].sample(local, time, self.looping);

            out_pose.joints[j as usize] = animated;
        }

        time
    }

    pub fn adjust_time_to_fit_range(&self, time: f32) -> f32 {
        let mut time = time;
        if self.looping {
            let duration = self.get_duration();
            if duration <= 0.0 {
                return 0.0;
            }

            time = (time - self.start_time) % duration;

            if time < 0.0 {
                time += duration;
            }
            time += self.start_time;
        } else {
            if time < self.start_time {
                time = self.start_time;
            }
            if time > self.end_time {
                time = self.end_time;
            }
        }

        time
    }

    pub fn re_calculate_duration(&mut self) {
        self.start_time = 0.0;
        self.end_time = 0.0;

        let mut start_set = false;
        let mut end_set = false;

        let track_len = self.tracks.len();

        for i in 0..track_len {
            if self.tracks[i].is_valid() {
                let start_time = self.tracks[i].get_start_time();
                let end_time = self.tracks[i].get_end_time();

                if start_time < self.start_time || !start_set {
                    self.start_time = start_time;
                    start_set = true;
                }

                if end_time > self.end_time || !end_set {
                    self.end_time = end_time;
                    end_set = true;
                }
            }
        }
    }

    pub fn get_duration(&self) -> f32 {
        self.end_time - self.start_time
    }
}
