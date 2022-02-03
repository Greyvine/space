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
    var faceIndex: f32;
    var maxAdjust: f32;
    var uv: vec2<f32>;
    if (rayAbs.z >= rayAbs.x && rayAbs.z >= rayAbs.y) {
        if (ray.z < 0.0) {
            faceIndex = 5.0;
        } else {
            faceIndex = 4.0;
        }
        maxAdjust = 0.5 / rayAbs.z;
        uv = vec2<f32>(ray.x * -sign(ray.z), -ray.y);
    }
    else if (rayAbs.y >= rayAbs.x) {
        if (ray.y < 0.0) {
            faceIndex = 3.0;
        } else {
            faceIndex = 2.0;
        }
        maxAdjust = 0.5 / ray.y;
        uv = vec2<f32>(ray.x * sign(ray.y), -ray.z);
    }
    else {
        if (ray.x < 0.0) {
            faceIndex = 1.0;
        } else {
            faceIndex = 0.0;
        }
        maxAdjust = 0.5 / ray.x;
        uv = vec2<f32>(ray.z, ray.y * -sign(ray.x));
    }
    return vec3<f32>(uv.x * maxAdjust + 0.5, uv.y * maxAdjust + 0.5, faceIndex);
}

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    let uvIndex = sampleCubeHacky(in.world_position);
    let uv = vec2<f32>(uvIndex.x, uvIndex.y);
    let array_index: i32 = i32(uvIndex.z);
    return textureSample(base_color_texture, base_color_sampler, uv, array_index);
}