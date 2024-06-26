struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-0.75, -0.75),
        vec2<f32>(0.75, -0.75),
        vec2<f32>(0.0, 0.75)
    );

    var output = VertexOutput;
    output.clip_position = vec4<f32>(positions[i], 0.0, 1.0);
    return output;
};

@fragment
fn fs_main(in: VertexOutput) -> vec4<f32> {
    return vec4<f32>(0.0, 0.4, 0.4, 1.0);
};