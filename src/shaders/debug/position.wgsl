struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) color: vec3<f32>,
    @location(3) uv: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec3<f32>,
}

struct Camera {
    view: mat4x4<f32>,
    position: vec3<f32>,
    direction: vec3<f32>,
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> uCamera: Camera;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
	var position = vec4<f32>(in.position, 1.0);
	position = (uCamera.projection * uCamera.view) * position;
	return VertexOutput(
		position,
		vec3<f32>(in.position.x, in.position.y, in.position.z)
	);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return vec4<f32>(in.color, 1.0);
}
