
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct SimpleMaterial {
    color: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> uniform_data: SimpleMaterial;

[[group(1), binding(1)]]
var texture: texture_2d<f32>;

[[group(1), binding(2)]]
var _sampler: sampler;

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var output_color = uniform_data.color;
    output_color = output_color * textureSample(texture, _sampler, input.uv);
    return output_color;
}
