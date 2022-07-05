
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

struct FoliageMaterial {
    color: vec4<f32>;
    velocity: vec2<f32>; 
};

[[group(1), binding(0)]]
var<uniform> uniform_data: FoliageMaterial;

[[group(1), binding(1)]]
var texture: texture_2d<f32>;

[[group(1), binding(2)]]
var _sampler: sampler;

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var output_color = uniform_data.color;
    return output_color;
}
