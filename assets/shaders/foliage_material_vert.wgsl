
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

struct FoliageMaterial {
    color: vec4<f32>;
    velocity: vec2<f32>; 
};

[[stage(vertex)]]
fn vertex(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(0.0, 1.0, 0.0, 1.0);
    out.uv = input.uv;
    return out;
}
