use std::error::Error;
use std::fs;
use std::path::Path;

pub struct GltfFile {
    /// parent folder holding gltf/glb assets
    folder: String,

    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
}

impl GltfFile {
    pub fn load_gltf(folder: &Path) -> Result<GltfFile, Box<dyn Error>> {
        if !folder.is_dir() {
            panic!("Provided path is not a directory");
        }

        let mut gltf_path = None;

        for entry in fs::read_dir(folder)? {
            let path = entry?.path();

            if let Some(ext) = path.extension() {
                if ext.eq("gltf") || ext.eq("glb") {
                    gltf_path = Some(path);
                }
            }
        }

        let gltf_path = gltf_path.ok_or_else(|| {
            panic!("No .gltf or .glb file found in the folder");
        })?;

        let folder = gltf_path
            .parent()
            .unwrap_or(folder)
            .to_string_lossy()
            .to_string();

        let (document, buffers, ..) = match gltf_path.extension().unwrap().to_str().unwrap() {
            "glb" => {
                let data = fs::read(&gltf_path)?;
                gltf::import_slice(&data)?
            }
            "gltf" => gltf::import(gltf_path)?,

            _ => unreachable!(),
        };

        Ok(GltfFile {
            folder,
            document,
            buffers,
        })
    }

    pub fn get_buffers(&self) -> &Vec<gltf::buffer::Data> {
        &self.buffers
    }

    pub fn get_folder(&self) -> &String {
        &self.folder
    }

    pub fn get_document(&self) -> &gltf::Document {
        &self.document
    }
}
