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
	@location(3) world_position: vec3<f32>,
}

struct Camera {
    view: mat4x4<f32>,
    position: vec3<f32>,
    direction: vec3<f32>,
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>,
}

struct LightingUniform {
	color: vec3<f32>,
	position: vec3<f32>,
	intensity: f32,
}

@group(0) @binding(0)
var<uniform> uTime: f32;

@group(0) @binding(7)
var<uniform> uLights: array<LightingUniform, 8>;

@group(1) @binding(0)
var<uniform> uCamera: Camera;

@group(2) @binding(0)
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
		in.position
	);
}

@group(3) @binding(0)
var tAlbedo: texture_2d<f32>;
@group(3) @binding(1)
var sAlbedo: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let uv = vec2<f32>(in.uv.x, 1.0 - in.uv.y);
	var albedo = textureSample(tAlbedo, sAlbedo, uv);

	if (albedo.a < 0.1) {
		discard;
	}

	var color = vec3<f32>(0.0, 0.0, 0.0);
	for (var i: u32 = 0; i < 8; i = i + 1) {
		let light = uLights[i];
		let lightDir = normalize(light.position - in.world_position);
		let diffuse = max(dot(normalize(in.normal), lightDir), 0.0);
		let lightColor = light.color * light.intensity * diffuse;
		color += lightColor * albedo.rgb;
	}

	let ambient = vec3<f32>(0.1, 0.1, 0.1);
	color = color + (ambient * albedo.rgb);

	return vec4<f32>(color, albedo.a);
}
