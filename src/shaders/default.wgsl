struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) color: vec3<f32>,
    @location(3) uv: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) normal: vec3<f32>,
	@location(1) color: vec3<f32>,
	@location(2) uv: vec2<f32>,
}

struct Camera {
	position: vec3<f32>,
	direction: vec3<f32>,
	viewproj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uTime: f32;

@group(0) @binding(1)
var<uniform> uCamera: Camera;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
	let position = vec4<f32>(in.position, 1.0);

	return VertexOutput(
		position,
		in.normal,
		in.color,
		in.uv,
	);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
	return vec4<f32>(in.color, 1.0);
}
