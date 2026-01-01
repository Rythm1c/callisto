struct DirectionalLight {
    direction: vec3<f32>,
    _pad0: f32,
    //padding

    color: vec3<f32>,
    _pad1: f32 //padding
}

struct FrameUniforms {
    view_proj: mat4x4<f32>,

    cam_pos: vec3<f32>,
    _pad0: f32,
    //padding

    sun: DirectionalLight
}

@group(0) @binding(0)
var<uniform> frame: FrameUniforms;
//-----------------------------------
//-----------------------------------

@group(1) @binding(0)
var<uniform> model: mat4x4<f32>;

//-----------------------------------
//-----------------------------------

struct Material {
    base_color_factor: vec4<f32>,
    metallic_factor: f32,
    roughness_factor: f32,
    ao: f32,
}

;

struct TextureFlags {
    has_base_color: u32,
    has_metallic_roughness: u32,
    has_normal: u32,
}

;

@group(2) @binding(0)
var<uniform> material: Material;
@group(2) @binding(1)
var<uniform> tex_flags: TextureFlags;
@group(2) @binding(2)
var base_color_tex: texture_2d<f32>;
@group(2) @binding(3)
var base_color_sampler: sampler;
@group(2) @binding(4)
var metallic_roughness_tex: texture_2d<f32>;
@group(2) @binding(5)
var metallic_roughness_sampler: sampler;
@group(2) @binding(6)
var normal_tex: texture_2d<f32>;
@group(2) @binding(7)
var normal_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var result: VertexOutput;
    result.normal = in.normal;
    result.clip_pos = frame.view_proj * model * vec4<f32>(in.position, 1.0);
    result.world_pos = (model * vec4<f32>(in.position, 1.0)).xyz;
    result.uv = in.uv;
    return result;
}

// Helper to check if a texture is present
fn has_tex(flag: u32) -> bool {
    return flag != 0u;
}

// Fresnel Schlick approximation
fn fresnel_schlick(cos_theta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(1.0 - cos_theta, 5.0);
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    // Base color
    var base_color = material.base_color_factor;
    if has_tex(tex_flags.has_base_color) {
        base_color = textureSample(base_color_tex, base_color_sampler, vertex.uv) * material.base_color_factor;
    }

    // Metallic-Roughness
    var metallic = material.metallic_factor;
    var roughness = material.roughness_factor;
    if has_tex(tex_flags.has_metallic_roughness) {
        let mr_sample = textureSample(metallic_roughness_tex, metallic_roughness_sampler, vertex.uv);
        metallic = mr_sample.b * material.metallic_factor;
        roughness = mr_sample.g * material.roughness_factor;
    }
    let ao = material.ao;

    // Normal
    var N = normalize(vertex.normal);
    if has_tex(tex_flags.has_normal) {
        let n_sample = textureSample(normal_tex, normal_sampler, vertex.uv).xyz * 2.0 - vec3<f32>(1.0);
        N = normalize(n_sample);
    }

    let V = normalize(frame.cam_pos - vertex.world_pos);
    let L = normalize(- frame.sun.direction);
    let H = normalize(L + V);

    // Lighting
    let light_color = frame.sun.color;
    let radiance = light_color;

    // Cook-Torrance BRDF
    let NdotL = max(dot(N, L), 0.0);
    let NdotV = max(dot(N, V), 0.0);
    let NdotH = max(dot(N, H), 0.0);
    let VdotH = max(dot(V, H), 0.0);

    // F0
    let F0 = mix(vec3<f32>(0.04), base_color.rgb, metallic);
    let F = fresnel_schlick(VdotH, F0);

    // Distribution GGX
    let alpha = roughness * roughness;
    let alpha2 = alpha * alpha;
    let denom = (NdotH * NdotH) * (alpha2 - 1.0) + 1.0;
    let D = alpha2 / (3.141592 * denom * denom);

    // Geometry Schlick-GGX
    let k = (alpha + 1.0) * (alpha + 1.0) / 8.0;
    let G_V = NdotV / (NdotV * (1.0 - k) + k);
    let G_L = NdotL / (NdotL * (1.0 - k) + k);
    let G = G_V * G_L;

    // Specular
    let numerator = D * G * F;
    let denominator = 4.0 * NdotV * NdotL + 0.001;
    let specular = numerator / denominator;

    // kS is energy conservation, kD is diffuse
    let kS = F;
    let kD = (vec3<f32>(1.0) - kS) * (1.0 - metallic);

    // Lambertian diffuse
    let diffuse = kD * base_color.rgb / 3.141592;

    // Final color
    let color = (diffuse + specular) * radiance * NdotL;

    // Ambient
    let ambient = vec3<f32>(0.03) * base_color.rgb * ao;

    let out_color = ambient + color;
    return vec4<f32>(out_color, base_color.a);
}

@fragment
fn fs_wire(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}