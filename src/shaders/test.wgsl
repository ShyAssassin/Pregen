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

@group(0) @binding(0)
var<uniform> uTime: f32;

struct Camera {
    view: mat4x4<f32>,
    position: vec3<f32>,
    direction: vec3<f32>,
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> uCamera: Camera;

@group(2) @binding(0)
var tAlbedo: texture_2d<f32>;
@group(2) @binding(1)
var sAlbedo: sampler;

@group(3) @binding(0)
var<uniform> uModel: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
	var position = vec4<f32>(in.position, 1.0);
	position = (uCamera.projection * uCamera.view * uModel) * position;
	return VertexOutput(
		position,
		in.normal,
		in.color,
		in.uv,
	);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let albedo = textureSample(tAlbedo, sAlbedo, in.uv);
	return vec4<f32>(albedo);
}
