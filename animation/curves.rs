///bezier curves
pub struct Bezier<T> {
    ///point 1
    pub p1: T,
    ///control point 1
    pub c1: T,
    ///point 2
    pub p2: T,
    ///control point 2
    pub c2: T,
}

/// linear interpolation function  
/// s: starting value  
/// e: ending value  
/// pct: percentage  
/// TODO: find a better place to keep this function
pub fn lerp(s: f32, e: f32, pct: f32) -> f32 {
    s + (e - s) * pct
}

#[allow(dead_code)]
pub fn hermite(t: f32, p1: f32, s1: f32, p2: f32, s2: f32) -> f32 {
    p1 * ((1.0 + 2.0 * t) * ((1.0 - t) * (1.0 - t)))
        + s1 * (t * ((1.0 - t) * (1.0 - t)))
        + p2 * ((t * t) * (3.0 - 2.0 * t))
        + s2 * ((t * t) * (t - 1.0))
}

impl Bezier<f32> {
    pub fn interpolate(&mut self, t: f32) -> f32 {
        let a = lerp(self.p1, self.c1, t);
        let b = lerp(self.p2, self.c2, t);
        let c = lerp(self.c1, self.c2, t);

        let d = lerp(a, c, t);
        let e = lerp(c, b, t);

        lerp(d, e, t)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Interpolation {
    Constant,
    Linear,
    Cubic,
}
