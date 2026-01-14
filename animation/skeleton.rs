use super::pose::Pose;

use math::mat4::Mat4;

/// rest pose joints are parallel to inverse bind pose which are also parallel to joint names
#[derive(Clone)]
pub struct Skeleton {
    pub rest_pose: Pose,
    pub inverse_bind_pose: Vec<Option<Mat4>>,
    pub joint_names: Vec<String>,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            rest_pose: Pose::new(),
            inverse_bind_pose: Vec::new(),
            joint_names: Vec::new(),
        }
    }
}
