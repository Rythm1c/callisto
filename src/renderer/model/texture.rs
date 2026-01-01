use std::path::Path;

use image::GenericImageView;

use crate::renderer::model::importer::GltfFile;

#[derive(Clone)]
pub struct Texture {
    handle: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub fn get_handle(&self) -> &wgpu::Texture {
        &self.handle
    }

    pub fn get_view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn get_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn from_gltf(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &gltf::Texture,
        file: &GltfFile,
        label: Option<&str>,
    ) -> Self {
        let data = texture_data_from_gltf(texture, file);

        Self::upload(device, queue, label, &data)
    }

    fn upload(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        data: &TextureData,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: data.width,
            height: data.height,
            depth_or_array_layers: 1,
        };
        let handle = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Ensure pixels are RGBA8 (4 bytes per pixel)
        let rgba_pixels = match data.format {
            image::ColorType::Rgba8 => data.pixels.clone(),
            image::ColorType::Rgb8 => {
                // Convert RGB8 to RGBA8 by adding alpha channel
                let mut rgba = Vec::with_capacity(data.pixels.len() / 3 * 4);
                for chunk in data.pixels.chunks(3) {
                    rgba.push(chunk[0]);
                    rgba.push(chunk[1]);
                    rgba.push(chunk[2]);
                    rgba.push(255); // Opaque alpha
                }
                rgba
            }
            _ => panic!("color type not supported yet!"),
        };

        queue.write_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &handle,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * data.width),
                rows_per_image: Some(data.height),
            },
            size,
        );

        let view = handle.create_view(&wgpu::wgt::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::wgt::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            handle,
            view,
            sampler,
        }
    }
}

impl Texture {
    /// Create a 1x1 white texture (for base color fallback)
    pub fn fallback_white(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let data = TextureData {
            width: 1,
            height: 1,
            pixels: vec![255, 255, 255, 255],
            format: image::ColorType::Rgba8,
        };
        Self::upload(device, queue, Some("fallback_white"), &data)
    }

    /// Create a 1x1 neutral metallic-roughness texture (R=1, G=1, B=1, A=1)
    pub fn fallback_metallic_roughness(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let data = TextureData {
            width: 1,
            height: 1,
            pixels: vec![255, 255, 255, 255],
            format: image::ColorType::Rgba8,
        };
        Self::upload(device, queue, Some("fallback_metallic_roughness"), &data)
    }

    /// Create a 1x1 normal map texture (normal = (0,0,1) in tangent space, encoded as (128,128,255))
    pub fn fallback_normal(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let data = TextureData {
            width: 1,
            height: 1,
            pixels: vec![128, 128, 255, 255],
            format: image::ColorType::Rgba8,
        };
        Self::upload(device, queue, Some("fallback_normal"), &data)
    }
}

fn texture_data_from_gltf(texture: &gltf::Texture, file: &GltfFile) -> TextureData {
    let src = texture.source().source();

    match src {
        gltf::image::Source::Uri { uri, .. } => {
            let parent = Path::new(&file.get_folder()[..]);
            TextureData::from_path(parent.join(uri).as_path())
        }

        gltf::image::Source::View { view, mime_type } => {
            let buffer = &file.get_buffers()[view.buffer().index()];
            let start = view.offset();
            let end = start + view.length();
            let image_bytes = &buffer[start..end];

            match mime_type {
                "image/jpeg" => {
                    TextureData::from_memory(image_bytes.to_vec(), image::ImageFormat::Jpeg)
                }
                "image/png" => {
                    TextureData::from_memory(image_bytes.to_vec(), image::ImageFormat::Png)
                }
                _ => panic!("unsupported image type"),
            }
        }
    }
}

#[derive(Clone)]
struct TextureData {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    format: image::ColorType,
}

impl TextureData {
    fn from_path(path: &Path) -> Self {
        let img = image::open(path)
            .expect(format!("unable to open texture in {}", path.to_string_lossy()).as_str());

        let (width, height) = img.dimensions();
        let format = img.color();
        let pixels = img.into_bytes();

        Self {
            width,
            height,
            pixels,
            format,
        }
    }

    pub fn from_memory(data: Vec<u8>, format: image::ImageFormat) -> Self {
        let img = image::load_from_memory_with_format(&data, format)
            .expect("failed to load image from memory!");

        match format {
            image::ImageFormat::Jpeg => {
                let rgb = img.to_rgb8();

                let (w, h) = rgb.dimensions();
                TextureData {
                    width: w,
                    height: h,
                    pixels: data,
                    format: image::ColorType::Rgb8,
                }
            }
            image::ImageFormat::Png => {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                TextureData {
                    width: w,
                    height: h,
                    pixels: data,
                    format: image::ColorType::Rgba8,
                }
            }

            _ => panic!("Unsupported image format"),
        }
    }
}
