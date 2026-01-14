// _______________________________________________________________________________________________________
// _______________________________________________________________________________________________________
// got alot of help from the "gabor szauer - hands on c++ game animation programming packt" book
// most of this is just the books code translated to rust with a few changes here and there.
// tried my best to translate the c++ to rust so maybe a few minor rough edges here and there
// but hey if it works, it works

use std::ops::*;

use super::curves;
use super::frame;

use math::quaternion::Quat;
use math::vec3::Vec3;

#[derive(Clone)]
/// animation for a single node(joint, bone, which ever you'd prefer to call it)
pub struct Track<const N: usize> {
    pub frames: Vec<frame::Frame<N>>,
    pub interpolation: curves::Interpolation,
}

#[allow(unused)]
pub type ScalarTrack = Track<1>;
pub type VectorTrack = Track<3>;
pub type QuatTrack = Track<4>;

impl<const N: usize> Track<N> {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            interpolation: curves::Interpolation::Cubic,
        }
    }

    pub fn get_start_time(&self) -> f32 {
        self.frames[0].time
    }
    pub fn get_end_time(&self) -> f32 {
        self.frames.last().unwrap().time
    }
    pub fn sample<T>(&self, time: f32, looping: bool) -> T
    where
        T: Cast<T>
            + Interpolate<T>
            + AdjustHermiteResult<T>
            + Neighborhood<T>
            + Add<T, Output = T>
            + Mul<f32, Output = T>,
    {
        match self.interpolation {
            curves::Interpolation::Cubic => self.sample_cubic(time, looping),
            curves::Interpolation::Linear => self.sample_linear(time, looping),
            curves::Interpolation::Constant => self.sample_constant(time, looping),
        }
    }

    fn sample_constant<T: Cast<T>>(&self, time: f32, looping: bool) -> T {
        let frame = self.frame_index(time, looping).unwrap();

        T::cast(&self.frames[frame].m_value)
    }

    fn sample_linear<T: Cast<T> + Interpolate<T>>(&self, time: f32, looping: bool) -> T {
        let frame = self.frame_index(time, looping).unwrap();
        let next_frame = frame + 1;

        let track_time = self.adjust_to_fit_track(time, looping);
        let this_time = self.frames[frame].time;
        let frame_delta = self.frames[next_frame].time - this_time;

        if frame_delta <= 0.0 {
            println!("error while linear sampling occured!");
        }

        let t = (track_time - this_time) / frame_delta;

        let start = T::cast(&self.frames[frame].m_value);
        let end = T::cast(&self.frames[next_frame].m_value);

        T::interpolate(&start, &end, t)
    }
    fn sample_cubic<T>(&self, time: f32, looping: bool) -> T
    where
        T: Cast<T>
            + Interpolate<T>
            + AdjustHermiteResult<T>
            + Neighborhood<T>
            + Add<T, Output = T>
            + Mul<f32, Output = T>,
    {
        let frame = self.frame_index(time, looping).unwrap();
        let next_frame = frame + 1;

        let track_time = self.adjust_to_fit_track(time, looping);
        let this_time = self.frames[frame].time;
        let frame_delta = self.frames[next_frame].time - this_time;

        if frame_delta <= 0.0 {
            println!("error while cubic sampling occured!");
        }

        let t = (track_time - this_time) / frame_delta;

        let point1 = T::cast(&self.frames[frame].m_value);
        let mut slope1: [f32; N] = [0.0; N];
        for i in 0..N {
            slope1[i] = self.frames[frame].m_out[i] * frame_delta
        }
        let slope1 = T::cast(&slope1);

        let point2 = T::cast(&self.frames[next_frame].m_value);
        let mut slope2: [f32; N] = [0.0; N];
        for i in 0..N {
            slope2[i] = self.frames[next_frame].m_in[i] * frame_delta
        }
        let slope2 = T::cast(&slope2);

        Self::hermite(t, point1, slope1, point2, slope2)
    }

    pub fn frame_index(&self, _time: f32, looping: bool) -> Result<usize, String> {
        let mut time = _time;
        let len = self.frames.len();

        if len <= 0 {
            // there has been an error
            return Err(String::from("error finding frame index!"));
        }

        if looping {
            let start_time = self.get_start_time();
            let end_time = self.get_end_time();
            let duration = end_time - start_time;

            time = (time - start_time) % duration;

            if time < 0.0 {
                time += duration;
            }
            time += start_time;
        } else {
            if time <= self.get_start_time() {
                return Ok(0);
            }
            if time >= self.frames[len - 2].time {
                return Ok(len - 2);
            }
        }

        for i in (0..len).rev() {
            if time >= self.frames[i].time {
                return Ok(i);
            }
        }

        // there has been an error
        Err(String::from("error finding frame index!"))
    }

    pub fn adjust_to_fit_track(&self, time: f32, looping: bool) -> f32 {
        let len = self.frames.len();
        let mut time = time;
        if len <= 1 {
            return 0.0;
        }

        let start_time = self.get_start_time();
        let end_time = self.get_end_time();
        let duration = end_time - start_time;

        if duration <= 0.0 {
            return 0.0;
        }
        if looping {
            time = (time - start_time) % duration;
            if time < 0.0 {
                time += duration;
            }

            time += start_time;
        } else {
            if time <= start_time {
                time = start_time;
            }
            if time >= end_time {
                time = end_time;
            }
        }

        time
    }

    pub fn hermite<T>(t: f32, p1: T, s1: T, _p2: T, s2: T) -> T
    where
        T: AdjustHermiteResult<T> + Neighborhood<T> + Add<T, Output = T> + Mul<f32, Output = T>,
    {
        let tt = t * t;
        let ttt = tt * t;

        let mut p2 = _p2;
        T::neighborhood(&p1, &mut p2);

        let h1 = 2.0 * ttt - 3.0 * tt + 1.0;
        let h2 = -2.0 * ttt + 3.0 * tt;
        let h3 = ttt - 2.0 * tt + t;
        let h4 = ttt - tt;
        let result = p1 * h1 + p2 * h2 + s1 * h3 + s2 * h4;

        T::adjust_hermite_result(&result)
    }
}
pub trait AdjustHermiteResult<T> {
    fn adjust_hermite_result(value: &T) -> T;
}

impl AdjustHermiteResult<f32> for f32 {
    fn adjust_hermite_result(value: &f32) -> f32 {
        *value
    }
}
impl AdjustHermiteResult<Vec3> for Vec3 {
    fn adjust_hermite_result(value: &Vec3) -> Vec3 {
        *value
    }
}
impl AdjustHermiteResult<Quat> for Quat {
    fn adjust_hermite_result(value: &Quat) -> Quat {
        value.unit()
    }
}

pub trait Neighborhood<T> {
    fn neighborhood(a: &T, b: &mut T);
}

impl Neighborhood<f32> for f32 {
    fn neighborhood(_a: &f32, _b: &mut f32) { /* do nothing...*/
    }
}
impl Neighborhood<Vec3> for Vec3 {
    fn neighborhood(_a: &Vec3, _b: &mut Vec3) { /* do nothing...*/
    }
}
impl Neighborhood<Quat> for Quat {
    fn neighborhood(a: &Quat, b: &mut Quat) {
        if Quat::dot(a, b) < 0.0 {
            *b = -*b;
        }
    }
}

pub trait Cast<T> {
    fn cast(value: &[f32]) -> T;
}
impl Cast<f32> for f32 {
    fn cast(value: &[f32]) -> f32 {
        value[0]
    }
}

impl Cast<Vec3> for Vec3 {
    fn cast(value: &[f32]) -> Vec3 {
        Vec3::new(value[0], value[1], value[2])
    }
}

impl Cast<Quat> for Quat {
    fn cast(value: &[f32]) -> Quat {
        Quat::new(value[0], value[1], value[2], value[3])
    }
}

pub trait Interpolate<T> {
    fn interpolate(a: &T, b: &T, c: f32) -> T;
}

impl Interpolate<f32> for f32 {
    fn interpolate(a: &f32, b: &f32, c: f32) -> f32 {
        a + (b - a) * c
    }
}
impl Interpolate<Vec3> for Vec3 {
    fn interpolate(a: &Vec3, b: &Vec3, c: f32) -> Vec3 {
        a.mix(*b, c)
    }
}
impl Interpolate<Quat> for Quat {
    fn interpolate(a: &Quat, b: &Quat, c: f32) -> Quat {
        let mut result = a.nlerp(*b, c);
        if Quat::dot(a, b) < 0.0 {
            result = a.nlerp(-*b, c);
        }
        //no need for result.unit() call
        //nlerp function already normalizes
        result
    }
}
