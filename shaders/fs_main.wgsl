struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0)
var ray_image : texture_2d<f32>;

@group(0) @binding(1)
var ray_sampler : sampler;

@fragment
fn fs_main(in : VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(ray_image, ray_sampler, in.uv);

    return vec4<f32>(color.rgb, 1);
}
