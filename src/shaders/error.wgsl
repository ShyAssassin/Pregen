struct VertexOutput {
	@location(0) uv: vec2<f32>,
	@builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    let uv = vec2<f32>(f32(index >> 1u), f32(index & 1u)) * 2.0;
    let pos = vec3<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 0.0);

    return VertexOutput (
        vec2<f32>(uv.x, uv.y),
        vec4<f32>(pos.x, pos.y, pos.z, 1.0),
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    const frequency = 8.0;
    let scaledUV = in.uv * frequency;
    if (i32(floor(scaledUV.x) + floor(scaledUV.y)) % 2) == 1 {
        return vec4<f32>(1.0, 0.0, 1.0, 1.0);
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    };
}
