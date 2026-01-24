struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

const POSITION: array<vec4<f32>, 3> = array<vec4<f32>, 3>(
    vec4<f32>(-1, 3, 0, 1),
    vec4<f32>(3, -1, 0, 1),
    vec4<f32>(-1, -1, 0, 1),
);
const UV: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(0, 2),
    vec2<f32>(2, 0),
    vec2<f32>(0, 0),
);

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var out: VertexOutput;

    out.position = POSITION[idx];
    out.uv = UV[idx];

    return out;
}
