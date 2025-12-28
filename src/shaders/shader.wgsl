struct Camera {
    view_proj: mat4x4<f32>
}

struct Light {
    direction: vec3<f32>,
    color: vec3<f32>
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> model: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,

}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var result: VertexOutput;
    result.normal = in.normal;
    result.position = camera.view_proj * vec4<f32>(in.position, 1.0);
    return result;
}

/* @group(0) @binding(1)
var r_color: texture_2d<u32>; */

struct Material {
    base_color: vec3<f32>,
    roughness: f32,
    metallic: f32,
    ocllusion: f32,
}

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {

    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    let direction = vec3<f32>(- 1.0, - 1.0, 1.0);

    let n = normalize(vertex.normal);
    let l = normalize(- direction);
    let diff = max(dot(n, l), 0.0);

    let color = diff * light_color;
    return vec4<f32>(color, 1.0);
}

@fragment
fn fs_wire(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}