struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var pos = vec2<f32>(0.0, 0.0);
    var color = vec3<f32>(0.0, 0.0, 0.0);
    if (index == 0u) {
        pos = vec2<f32>(-0.5, -0.5);
        color = vec3<f32>(0.0, 1.0, 0.0);
    } else if (index == 1u) {
        pos = vec2<f32>(0.5, -0.5);
        color = vec3<f32>(0.0, 0.0, 1.0);
    } else {
        pos = vec2<f32>(0.0, 0.5);
        color = vec3<f32>(1.0, 0.0, 0.0);
    }

    return VertexOutput(
        vec4<f32>(pos, 0.0, 1.0),
        color.xyz
    );
}

@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color.rgb, 1.0);
}
