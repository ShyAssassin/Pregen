struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    let x = f32(1 - i32(index)) * 0.5;
    let y = f32(i32(index & 1u) * 2 - 1) * 0.5;
    let position = vec4<f32>(x, y, 0.0, 1.0);

    var color = vec3<f32>(0.0, 0.0, 0.0);
    switch(index) {
        case 0u: { color = vec3<f32>(1.0, 0.0, 0.0); }
        case 1u: { color = vec3<f32>(0.0, 1.0, 0.0); }
        case 2u: { color = vec3<f32>(0.0, 0.0, 1.0); }
        default: { color = vec3<f32>(1.0, 1.0, 1.0); }
    }

    return VertexOutput(
        position,
        color,
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
