struct CustomMaterial {
    color: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;

[[group(1), binding(1)]]
var base_color_texture: texture_2d_array<f32>;

[[group(1), binding(2)]]
var base_color_sampler: sampler;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[builtin(position)]] frag_coord: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

fn sampleCubeHacky(ray: vec4<f32>) -> vec3<f32> {
    let rayAbs = abs(ray);
    var array_index: i32;
    var maxAdjust: f32;
    var uv: vec2<f32>;
    if (rayAbs.z >= rayAbs.x && rayAbs.z >= rayAbs.y) {
        if (ray.z < 0.0) {
            array_index = 5;
        } else {
            array_index = 4;
        }
        maxAdjust = 0.5 / rayAbs.z;
        uv = vec2<f32>(ray.x * -sign(ray.z), -ray.y);
    }
    else if (rayAbs.y >= rayAbs.x) {
        if (ray.y < 0.0) {
            array_index = 3;
        } else {
            array_index = 2;
        }
        maxAdjust = 0.5 / ray.y;
        uv = vec2<f32>(ray.x * sign(ray.y), -ray.z);
    }
    else {
        if (ray.x < 0.0) {
            array_index = 1;
        } else {
            array_index = 0;
        }
        maxAdjust = 0.5 / ray.x;
        uv = vec2<f32>(ray.z, ray.y * -sign(ray.x));
    }
    return vec3<f32>(uv.x * maxAdjust + 0.5, uv.y * maxAdjust + 0.5, f32(array_index));
}

#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
    [[location(3)]] vertex_position: vec4<f32>;
};

[[group(2), binding(0)]]
var<uniform> mesh: Mesh;


[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let uvIndex = sampleCubeHacky(in.vertex_position);
    let uv = vec2<f32>(uvIndex.x, uvIndex.y);
    let array_index: i32 = i32(uvIndex.z);
    return textureSample(base_color_texture, base_color_sampler, uv, array_index);
}

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.uv = vertex.uv;
    out.world_position = world_position;
    out.clip_position = view.view_proj * world_position;
    out.world_normal = mat3x3<f32>(
        mesh.inverse_transpose_model[0].xyz,
        mesh.inverse_transpose_model[1].xyz,
        mesh.inverse_transpose_model[2].xyz
    ) * vertex.normal;
    out.vertex_position = vec4<f32>(vertex.position, 1.0);

    return out;
}