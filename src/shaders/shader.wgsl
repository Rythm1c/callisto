struct DirectionalLight {
    direction: vec3<f32>,
    _pad0    : f32,//padding

    color    : vec3<f32>,
    _pad1    : f32 //padding
}

struct FrameUniforms {
    view_proj: mat4x4<f32>,

    cam_pos  : vec3<f32>,
    _pad0    : f32,//padding

    sun      : DirectionalLight
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
    base_color    : vec4<f32>,
    specular_color: vec3<f32>,
    shininess     : f32,
}

@group(2) @binding(0)
var<uniform> material: Material;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal  : vec3<f32>
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) world_pos     : vec3<f32>,
    @location(1) normal        : vec3<f32>,

}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var result: VertexOutput;
    result.normal = in.normal;
    result.clip_pos = frame.view_proj * vec4<f32>(in.position, 1.0);
    result.world_pos = (model * vec4<f32>(in.position, 1.0)).xyz;
    return result;
}

/* @group(0) @binding(1)
var r_color: texture_2d<u32>; */

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {

    //let light_color = vec3<f32>(1.0, 1.0, 1.0);
    //let direction = vec3<f32>(- 1.0, - 1.0, 1.0);

    let N = normalize(vertex.normal);
    let L = normalize(- frame.sun.direction);
    let V = normalize(frame.cam_pos - vertex.world_pos);
    let H = normalize(L + V);

    // Ambient
    let ambient_strength = 0.1;
    let ambient = ambient_strength * material.base_color.rgb;

    // diffuse
    let NdotL = max(dot(N, L), 0.0);
    let diffuse = material.base_color.rgb *
        frame.sun.color *
        NdotL;

    // specular
    let NdotH = max(dot(N, H), 0.0);
    let spec_factor = pow(NdotH, material.shininess);
    let specular = 
        material.specular_color *
        frame.sun.color *
        spec_factor;

    let color = specular + diffuse + specular;
    return vec4<f32>(color, 1.0);
}

@fragment
fn fs_wire(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}