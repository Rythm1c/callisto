use std::fs;
use std::io::Error;
use std::path::Path;

pub struct GltfFile {
    /// parent folder holding gltf/glb assets
    folder: String,

    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
}

impl GltfFile {
    pub fn load_gltf(folder: &Path) -> Result<GltfFile, Error> {
        let paths = fs::read_dir(folder).unwrap();

        let mut gltf_file = String::new();
        for entry in paths {
            let path = entry.unwrap().path();

            if let Some(extension) = path.extension() {
                if extension.eq("gltf") || extension.eq("glb") {
                    gltf_file = String::from(path.to_str().unwrap());
                }
            }
        }

        let folder = String::from(folder.to_str().unwrap());

        let (document, buffers, ..) = gltf::import(gltf_file.clone())
            .expect(format!("Failed to import gltf from {}", gltf_file).as_str());

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
