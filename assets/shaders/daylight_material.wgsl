
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

fn point_light(uv: vec2<f32>, pos: vec2<f32>, radius: f32, light_color: vec3<f32>) -> vec4<f32> {
    var dist = distance(uv, pos);
    var alpha = 1.0-min(dist/radius, 1.0);
    return vec4<f32>(light_color, alpha);
}

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var radius = 0.1;
    var circle_color = vec3<f32>(0.0, 1.0, 0.0);

    var output_color = point_light(input.uv, vec2<f32>(0.5, 0.5), radius, circle_color);
    return output_color;
}
