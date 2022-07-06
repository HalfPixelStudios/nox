
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct DaylightMaterial {
    color: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> uniform_data: DaylightMaterial;

[[group(1), binding(1)]]
var texture: texture_2d<f32>;

[[group(1), binding(2)]]
var _sampler: sampler;

// attenuation values can be obtained from https://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation
fn calculate_attenuation(uv: vec2<f32>, pos: vec2<f32>, attenuation_linear: f32, attenuation_quadratic: f32) -> f32 {
    var dist = distance(uv, pos);
    return 1.0 - 1.0/(1.0 + attenuation_linear*dist + attenuation_quadratic*pow(dist, 2.0));
}

fn point_light(uv: vec2<f32>, pos: vec2<f32>, radius: f32) -> f32 {
    var dist = distance(uv, pos);
    return min(dist/radius, 1.0);
}

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var max_alpha = 0.99; // darkest it's allowed to be
    var radius = 0.1;

    var output_color = vec3<f32>(uniform_data.color.x, uniform_data.color.y, uniform_data.color.z);
    var pos = vec2<f32>(input.world_position.x, input.world_position.y);
    var alpha = calculate_attenuation(pos, vec2<f32>(0.5, 0.5), 0.022, 0.0019);
    return vec4<f32>(output_color, clamp(alpha, 0.0, max_alpha));
}
