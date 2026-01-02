use super::importer::GltfFile;
use math::mat4::{Mat4, transpose};

#[derive(Clone)]
pub struct Skin {
    // node ids for the skin
    pub joints: Vec<usize>,
    pub inverse_bind_poses: Option<Vec<Mat4>>,
    pub skeleton: Option<usize>,
}

impl Skin {
    pub fn new() -> Self {
        Self {
            joints: Vec::new(),
            inverse_bind_poses: None,
            skeleton: None,
        }
    }

    pub fn from_gltf(skin: &gltf::Skin, file: &GltfFile) -> Self {
        let mut skeleton: Option<usize> = None;
        if let Some(s) = skin.skeleton() {
            skeleton = Some(s.index());
        }

        let reader = skin.reader(|buffer| Some(&file.get_buffers()[buffer.index()]));

        let mut inverse_bind_poses = None;
        if let Some(inverse_bind_mats) = reader.read_inverse_bind_matrices() {
            inverse_bind_poses = Some(
                inverse_bind_mats
                    .map(|inverse_mat| transpose(&Mat4::from(&inverse_mat)))
                    .collect::<Vec<Mat4>>(),
            );
        }

        Self {
            joints: skin
                .joints()
                .map(|joint| joint.index())
                .collect::<Vec<usize>>(),
            inverse_bind_poses,
            skeleton,
        }
    }
}
