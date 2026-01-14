use math::{mat4::Mat4, transform::Transform};

#[derive(Clone)]
pub struct Pose {
    pub joints: Vec<Transform>,
    pub parents: Vec<i32>,
}

impl Pose {
    pub fn new() -> Self {
        Self {
            joints: Vec::new(),
            parents: Vec::new(),
        }
    }
    pub fn get_global_tranform(&self, i: usize) -> Transform {
        let mut result = self.joints[i];
        let mut p = self.parents[i];

        //infinitly loop until a parent index of -1(root joint containing not parent) is found
        loop {
            if p < 0 {
                return result;
            }
            result = Transform::combine(&self.joints[p as usize], &result);
            p = self.parents[p as usize];
        }
    }
    pub fn resize(&mut self, new_len: usize) {
        self.parents.resize(new_len, -1);
        self.joints.resize(new_len, Transform::DEFAULT);
    }

    pub fn get_matrix_palette(&mut self, out: &mut Vec<Mat4>) {
        let len = self.joints.len();
        if out.len() != len {
            out.resize(len, Mat4::IDENTITY);
        }

        for i in 0..len {
            let t = &mut self.get_global_tranform(i);
            out[i] = t.to_mat();
        }
    }
}

impl PartialEq for Pose {
    fn eq(&self, other: &Self) -> bool {
        if self.joints.len() != other.joints.len() {
            return false;
        }
        if self.parents.len() != other.parents.len() {
            return false;
        }
        let len = self.joints.len();

        for i in 0..len {
            let this_local = &self.joints[i];
            let other_local = &other.joints[i];

            let this_parent = self.parents[i];
            let other_parent = other.parents[i];

            if this_parent != other_parent {
                return false;
            }
            if this_local.translation != other_local.translation {
                return false;
            }
            if this_local.orientation != other_local.orientation {
                return false;
            }
            if this_local.scaling != other_local.scaling {
                return false;
            }
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}
